use axum::{http::StatusCode, Json};
use diesel_async::pooled_connection::bb8::RunError;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use serde_json::{json, Value};
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct DieselError(diesel::result::Error);

impl From<diesel::result::Error> for DieselError {
    fn from(error: diesel::result::Error) -> Self {
        DieselError(error)
    }
}

impl Serialize for DieselError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl std::fmt::Display for DieselError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, ToSchema)]
pub enum CustomErrors<'a> {
    DieselError {
        /// diesel::result::Error
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

impl<'a> Serialize for CustomErrors<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CustomErrors::DieselError { error, message } => {
                let mut state = serializer.serialize_struct("DieselError", 3)?;
                state.serialize_field("status", &StatusCode::BAD_REQUEST.as_u16())?;
                state.serialize_field("error", &error.to_string())?;
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
                state.serialize_field("status", &status.as_u16())?;
                state.serialize_field("error", &error.to_string())?;
                if let Some(msg) = message {
                    state.serialize_field("extra", msg)?;
                }
                state.end()
            }
            CustomErrors::StringError { status, error } => {
                let mut state = serializer.serialize_struct("StringError", 2)?;
                state.serialize_field("status", &status.as_u16())?;
                state.serialize_field("error", error)?;
                state.end()
            }
            CustomErrors::PoolConnectionError(error) => {
                let mut state = serializer.serialize_struct("PoolConnectionError", 3)?;
                state.serialize_field("status", &StatusCode::INTERNAL_SERVER_ERROR.as_u16())?;
                state.serialize_field("error", &error.to_string())?;
                state.serialize_field("extra", "Failed to get a database connection")?;
                state.end()
            }
        }
    }
}
