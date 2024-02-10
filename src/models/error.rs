use axum::{http::StatusCode, Json};
use diesel_async::pooled_connection::bb8::RunError;
use serde_json::{json, Value};

#[derive(Debug)]
pub enum CustomErrors<'a> {
    DieselError {
        error: diesel::result::Error,
        message: Option<&'a str>,
    },
    Argon2Error {
        status: StatusCode,
        error: argon2::password_hash::Error,
        message: Option<&'a str>,
    },
    StringError {
        status: StatusCode,
        error: &'a str,
    },
    PoolConnectionError(RunError),
}

impl From<CustomErrors<'_>> for (StatusCode, Json<Value>) {
    fn from(err: CustomErrors<'_>) -> (StatusCode, Json<Value>) {
        match err {
            CustomErrors::DieselError { error, message } => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":error.to_string(), "extra":message})),
            ),
            CustomErrors::Argon2Error {
                status,
                error,
                message,
            } => (
                status,
                Json(json!({"error":error.to_string(), "extra":message})),
            ),
            CustomErrors::StringError { status, error } => {
                (status, Json(json!({"error":error.to_string()})))
            }
            CustomErrors::PoolConnectionError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"error":error.to_string(), "extra":"Failed to get a database connection"}),
                ),
            ),
        }
    }
}
