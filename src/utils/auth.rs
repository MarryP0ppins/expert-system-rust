use diesel::PgConnection;
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::Value,
};
use rocket_contrib::json;

use crate::services::user::get_user;

pub fn cookie_check(
    connection: &mut PgConnection,
    cookie: &CookieJar<'_>,
) -> Result<(), Custom<Value>> {
    match cookie
        .get_private("session_id")
        .map(|res| res.value().to_owned())
    {
        Some(res) => match get_user(connection, res.parse::<i32>().expect("Server Error")) {
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
