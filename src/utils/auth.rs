use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use diesel_async::AsyncPgConnection;
use tower_cookies::{Cookies, Key};

use crate::{constants::COOKIE_NAME, models::error::CustomErrors, services::user::get_user};

pub async fn cookie_check<'a>(
    connection: &'a mut AsyncPgConnection,
    cookie: Cookies,
    cookie_key: &'a Key,
) -> Result<(), CustomErrors> {
    match cookie
        .private(&cookie_key)
        .get(COOKIE_NAME)
        .map(|res| res.value().to_owned())
    {
        Some(res) => match get_user(connection, res.parse::<i32>().expect("Server Error")).await {
            Ok(_) => Ok(()),
            Err(err) => Err(CustomErrors::DieselError {
                error: err,
                message: Some("Invalid credentials provided".to_string()),
            }),
        },
        None => Err(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }),
    }
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Cant hash password")
        .to_string()
}

pub fn check_password(password: &str, actual_password: &str) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(&actual_password).expect("Cant parse actual password");
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
