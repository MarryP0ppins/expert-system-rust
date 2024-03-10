use axum::http::StatusCode;
use utoipa::ToSchema;

use super::response_body::ResponseBodyError;

#[derive(ToSchema)]
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

impl Into<ResponseBodyError> for CustomErrors {
    fn into(self) -> ResponseBodyError {
        match self {
            CustomErrors::DieselError { error, message } => ResponseBodyError {
                status: StatusCode::BAD_REQUEST,
                error: error.to_string(),
                extra: message,
            },
            CustomErrors::Argon2Error {
                status,
                error,
                message,
            } => ResponseBodyError {
                status,
                error: error.to_string(),
                extra: message,
            },
            CustomErrors::StringError { status, error } => ResponseBodyError {
                status,
                error,
                extra: None,
            },
            CustomErrors::PoolConnectionError(error) => ResponseBodyError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: error.to_string(),
                extra: Some("Failed to get a database connection".to_string()),
            },
        }
    }
}
