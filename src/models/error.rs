use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub enum CustomErrors {
    DieselError {
        #[schema(value_type=String)]
        error: diesel::result::Error,
        message: Option<String>,
    },
    Argon2Error {
        #[schema(value_type=u16)]
        status: StatusCode,
        #[schema(value_type=String)]
        error: argon2::password_hash::Error,
        message: Option<String>,
    },
    StringError {
        #[schema(value_type=u16)]
        status: StatusCode,
        error: String,
    },
    #[schema(value_type=String)]
    PoolConnectionError(diesel_async::pooled_connection::bb8::RunError),
}

impl IntoResponse for CustomErrors {
    fn into_response(self) -> Response {
        let response = match self {
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
            CustomErrors::StringError { status, error } => (status, Json(json!({"error":error}))),
            CustomErrors::PoolConnectionError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"error":error.to_string(), "extra":"Failed to get a database connection"}),
                ),
            ),
        };

        response.into_response()
    }
}

impl Serialize for CustomErrors {
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
