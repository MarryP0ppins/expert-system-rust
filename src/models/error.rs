use rocket::http::Status;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CustomError {
    pub status: Status,
    pub error: String,
}
