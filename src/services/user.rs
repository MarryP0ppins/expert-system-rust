use crate::{
    constants::COOKIE_NAME,
    models::{
        error::CustomErrors,
        user::{NewUser, UpdateUser, UpdateUserResponse, User, UserLogin, UserWithoutPassword},
    },
    schema::users::dsl::*,
    utils::auth::{check_password, hash_password},
};
use axum::http::StatusCode;
use diesel::{insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tower_cookies::{
    cookie::{
        time::{Duration, OffsetDateTime},
        SameSite,
    },
    Cookie, Cookies, Key,
};

pub async fn get_user(
    connection: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<UserWithoutPassword, Error> {
    Ok(users
        .find(user_id)
        .select((
            id,
            email,
            username,
            created_at,
            first_name,
            last_name,
            is_superuser,
        ))
        .first::<UserWithoutPassword>(connection)
        .await?)
}

pub async fn update_user(
    connection: &mut AsyncPgConnection,
    user_data: UpdateUserResponse,
    user_id: i32,
) -> Result<UserWithoutPassword, Error> {
    let new_user: User = update(users.find(user_id))
        .set::<UpdateUser>(UpdateUser {
            email: user_data.email,
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            password: user_data.new_password,
        })
        .get_result::<User>(connection)
        .await?;

    Ok(UserWithoutPassword {
        id: new_user.id,
        email: new_user.email,
        username: new_user.username,
        created_at: new_user.created_at,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        is_superuser: new_user.is_superuser,
    })
}

pub async fn create_user(
    connection: &mut AsyncPgConnection,
    user_info: NewUser,
    cookie: Cookies,
    cookie_key: &Key,
) -> Result<UserWithoutPassword, Error> {
    match insert_into(users)
        .values::<NewUser>(NewUser {
            password: hash_password(&user_info.password),
            ..user_info
        })
        .returning((
            id,
            email,
            username,
            created_at,
            first_name,
            last_name,
            is_superuser,
        ))
        .get_result::<UserWithoutPassword>(connection)
        .await
    {
        Ok(_user) => {
            cookie.private(cookie_key).add(
                Cookie::build((COOKIE_NAME, _user.id.to_string()))
                    .path("/")
                    .secure(true)
                    .http_only(false)
                    .same_site(SameSite::Strict)
                    .expires(OffsetDateTime::now_utc() + Duration::days(2))
                    .into(),
            );
            Ok(_user)
        }
        Err(err) => Err(err),
    }
}

pub async fn login_user(
    connection: &mut AsyncPgConnection,
    user_info: UserLogin,
    cookie: Cookies,
    cookie_key: &Key,
) -> Result<UserWithoutPassword, CustomErrors> {
    let _user = users
        .filter(email.eq(user_info.email))
        .first::<User>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: Some("Invalid credantials provided".to_string()),
        })?;

    let null_cookie = Cookie::build((COOKIE_NAME, ""))
        .path("/")
        .expires(OffsetDateTime::now_utc());
    cookie.private(cookie_key).add(null_cookie.into());

    match check_password(&user_info.password, &_user.password) {
        Ok(_) => {
            cookie.private(cookie_key).add(
                Cookie::build((COOKIE_NAME, _user.id.to_string()))
                    .path("/")
                    .secure(true)
                    .http_only(false)
                    .same_site(SameSite::Strict)
                    .expires(OffsetDateTime::now_utc() + Duration::days(2))
                    .into(),
            );
            Ok(UserWithoutPassword {
                id: _user.id,
                email: _user.email,
                username: _user.username,
                created_at: _user.created_at,
                first_name: _user.first_name,
                last_name: _user.last_name,
                is_superuser: _user.is_superuser,
            })
        }
        Err(err) => Err(CustomErrors::Argon2Error {
            status: StatusCode::BAD_REQUEST,
            error: err,
            message: Some("Invalid credantials provided".to_string()),
        }),
    }
}
