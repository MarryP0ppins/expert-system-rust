use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub enum CustomErrors {
    SeaORMError {
        #[schema(value_type=String)]
        error: DbErr,
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
    AesGsmError {
        #[schema(value_type=String)]
        error: aes_gcm_siv::Error,
        message: Option<String>,
    },
}

impl Serialize for CustomErrors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CustomErrors::SeaORMError { error, message } => {
                let mut state = serializer.serialize_struct("SeaORMError", 3)?;
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
            CustomErrors::AesGsmError { error, message } => {
                let mut state = serializer.serialize_struct("AesGsmError", 3)?;
                state.serialize_field("status", &StatusCode::BAD_REQUEST.as_u16())?;
                state.serialize_field("error", &error.to_string())?;
                if let Some(msg) = message {
                    state.serialize_field("extra", msg)?;
                }
                state.end()
            }
        }
    }
}

impl IntoResponse for CustomErrors {
    fn into_response(self) -> Response {
        let response = match self {
            CustomErrors::SeaORMError { error, message } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": StatusCode::BAD_REQUEST.as_u16(),
                    "error": error.to_string(),
                    "extra": message,
                })),
            ),
            CustomErrors::Argon2Error {
                status,
                error,
                message,
            } => (
                status,
                Json(json!({
                    "status":status.as_u16(),
                    "error": error.to_string(),
                    "extra": message,
                })),
            ),
            CustomErrors::StringError { status, error } => (
                status,
                Json(json!({
                    "status":status.as_u16(),
                    "error":error
                })),
            ),
            CustomErrors::AesGsmError { error, message } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": StatusCode::BAD_REQUEST.as_u16(),
                    "error": error.to_string(),
                    "extra": message,
                })),
            ),
        };
        response.into_response()
    }
}
