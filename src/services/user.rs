use crate::{
    models::user::{NewUser, User, UserLogin, UserWithoutPassword},
    schema::users::dsl::*,
};
use diesel::{insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::Value,
};
use rocket_contrib::json;

pub async fn get_user(
    connection: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<UserWithoutPassword, Error> {
    match users
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
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn create_user(
    connection: &mut AsyncPgConnection,
    user_info: NewUser,
) -> Result<UserWithoutPassword, Error> {
    match insert_into(users)
        .values::<NewUser>(user_info)
        .returning((
            id,
            email,
            username,
            created_at,
            first_name,
            last_name,
            is_superuser,
        ))
        .get_result(connection)
        .await
    {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}

pub async fn login_user(
    connection: &mut AsyncPgConnection,
    user_info: UserLogin,
    cookie: &CookieJar<'_>,
) -> Result<UserWithoutPassword, Custom<Value>> {
    let _user: User;
    match users
        .filter(email.eq(user_info.email))
        .first::<User>(connection)
        .await
    {
        Ok(result) => _user = result,
        Err(err) => {
            return Err(Custom(
                Status::BadRequest,
                json!({"error":err.to_string(), "message":"Invalid credentials provided"}).into(),
            ))
        }
    }

    if _user.password == user_info.password {
        cookie.remove_private("session_id");
        cookie.add_private(("session_id", _user.id.to_string()));
    }

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
