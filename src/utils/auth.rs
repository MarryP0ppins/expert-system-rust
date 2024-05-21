use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use diesel_async::AsyncPgConnection;
use tower_cookies::{Cookies, Key};

use crate::{
    constants::COOKIE_NAME,
    models::{error::CustomErrors, user::User},
    schema::users::dsl::*,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn cookie_check<'a>(
    connection: &'a mut AsyncPgConnection,
    cookie: Cookies,
    cookie_key: &'a Key,
) -> Result<User, CustomErrors> {
    let user_id = cookie
        .private(&cookie_key)
        .get(COOKIE_NAME)
        .map(|res| res.value().to_owned())
        .and_then(|res| Some(res.parse::<i32>().expect("Server Error")))
        .ok_or_else(|| CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })?;

    Ok(users
        .find(user_id)
        .first::<User>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: Some("Invalid credentials provided".to_string()),
        })?)
}

pub async fn password_check<'a>(
    connection: &'a mut AsyncPgConnection,
    cookie: Cookies,
    cookie_key: &'a Key,
    password_to_check: &'a str,
) -> Result<User, CustomErrors> {
    let user_cookie = cookie_check(connection, cookie, cookie_key).await?;

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
