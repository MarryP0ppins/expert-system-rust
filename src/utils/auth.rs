use crate::{
    constants::COOKIE_NAME,
    entity::users::{Entity as UserEntity, Model as UserModel},
    models::error::CustomErrors,
};
use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use sea_orm::*;
use tower_cookies::{Cookies, Key};

pub async fn cookie_check<'a, C>(
    db: &'a C,
    cookie: Cookies,
    cookie_key: &'a Key,
) -> Result<UserModel, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user_id = cookie
        .private(&cookie_key)
        .get(COOKIE_NAME)
        .map(|res| res.value().to_owned())
        .and_then(|res| res.parse::<i32>().ok())
        .ok_or(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })?;

    let user = UserEntity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?
        .ok_or(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Invalid credentials provided".to_string(),
        })?;
    Ok(user)
}

pub async fn password_check<'a, C>(
    db: &'a C,
    cookie: Cookies,
    cookie_key: &'a Key,
    password_to_check: &'a str,
) -> Result<UserModel, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user_cookie = cookie_check(db, cookie, cookie_key).await?;

    Ok(check_password(password_to_check, &user_cookie.password)
        .and(Ok(user_cookie))
        .map_err(|err| CustomErrors::Argon2Error {
            status: StatusCode::BAD_REQUEST,
            error: err,
            message: Some("Неверный пароль".to_owned()),
        })?)
}

pub fn hash_password(new_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(new_password.as_bytes(), &salt)
        .expect("Cant hash password")
        .to_string()
}

pub fn check_password(password_to_check: &str, actual_password: &str) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(&actual_password).expect("Cant parse actual password");
    Argon2::default().verify_password(password_to_check.as_bytes(), &parsed_hash)
}
