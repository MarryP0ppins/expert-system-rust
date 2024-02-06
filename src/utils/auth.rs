use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use diesel_async::AsyncPgConnection;
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::Value,
};
use rocket_contrib::json;

use crate::services::user::get_user;

pub async fn cookie_check(
    connection: &mut AsyncPgConnection,
    cookie: &CookieJar<'_>,
) -> Result<(), Custom<Value>> {
    match cookie
        .get_private("session_id")
        .map(|res| res.value().to_owned())
    {
        Some(res) => match get_user(connection, res.parse::<i32>().expect("Server Error")).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Custom(
                Status::BadRequest,
                json!({"error":err.to_string(), "message":"Invalid credentials provided"}).into(),
            )),
        },
        None => Err(Custom(
            Status::Unauthorized,
            json!({"error":"Not authorized"}).into(),
        )),
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
