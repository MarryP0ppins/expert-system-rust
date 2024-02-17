use axum::{http::StatusCode, Json};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use serde_json::{json, Value};
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub enum CustomErrors<'a> {
    DieselError {
        error: String,
        message: Option<&'a str>,
    },
    Argon2Error {
        status: u16,
        error: String,
        message: Option<&'a str>,
    },
    StringError {
        status: u16,
        error: &'a str,
    },
    PoolConnectionError(String),
}

impl From<CustomErrors<'_>> for (StatusCode, Json<Value>) {
    fn from(err: CustomErrors<'_>) -> (StatusCode, Json<Value>) {
        match err {
            CustomErrors::DieselError { error, message } => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":error, "extra":message})),
            ),
            CustomErrors::Argon2Error {
                status,
                error,
                message,
            } => (
                StatusCode::from_u16(status).expect("Wrong status code"),
                Json(json!({"error":error, "extra":message})),
            ),
            CustomErrors::StringError { status, error } => (
                StatusCode::from_u16(status).expect("Wrong status code"),
                Json(json!({"error":error})),
            ),
            CustomErrors::PoolConnectionError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":error, "extra":"Failed to get a database connection"})),
            ),
        }
    }
}

impl<'a> Serialize for CustomErrors<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CustomErrors::DieselError { error, message } => {
                let mut state = serializer.serialize_struct("DieselError", 3)?;
                state.serialize_field("status", &StatusCode::BAD_REQUEST.as_u16())?;
                state.serialize_field("error", error)?;
                if let Some(msg) = message {
                    state.serialize_field("extra", msg)?;
                }
                state.end()
            }
            CustomErrors::Argon2Error {
                status,
                error,
                message,
            } => {
                let mut state = serializer.serialize_struct("Argon2Error", 3)?;
                state.serialize_field("status", status)?;
                state.serialize_field("error", error)?;
                if let Some(msg) = message {
                    state.serialize_field("extra", msg)?;
                }
                state.end()
            }
            CustomErrors::StringError { status, error } => {
                let mut state = serializer.serialize_struct("StringError", 2)?;
                state.serialize_field("status", status)?;
                state.serialize_field("error", error)?;
                state.end()
            }
            CustomErrors::PoolConnectionError(error) => {
                let mut state = serializer.serialize_struct("PoolConnectionError", 3)?;
                state.serialize_field("status", &StatusCode::INTERNAL_SERVER_ERROR.as_u16())?;
                state.serialize_field("error", error)?;
                state.serialize_field("extra", "Failed to get a database connection")?;
                state.end()
            }
        }
    }
}
