use super::{
    answer::Answer, attribute::AttributeWithAttributeValues, attribute_value::AttributeValue,
    error::CustomErrors, system::System,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseBodyError {
    pub status: u16,
    pub error: String,
    pub extra: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[aliases(
    ResponseBodyEmpty = ResponseBody<String>,
    ResponseBodyAnswer = ResponseBody<Vec<Answer>>,
    ResponseBodyAttributeValue = ResponseBody<Vec<AttributeValue>>,
    ResponseBodyAttribute = ResponseBody<Vec<AttributeWithAttributeValues>>,
    ResponseBodySystem = ResponseBody<System>,
    ResponseBodySystems = ResponseBody<Vec<System>>
)]
pub struct ResponseBody<T> {
    pub succsess: bool,
    pub data: Option<T>,
    pub error: Option<ResponseBodyError>,
}

impl<T> ResponseBody<T> {
    pub fn unauthorized_example() -> ResponseBody<T> {
        ResponseBody::<T> {
            succsess: false,
            data: None,
            error: Some(ResponseBodyError {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                error: "Not authorized".to_string(),
                extra: None,
            }),
        }
    }
}

impl<T> From<CustomErrors> for ResponseBody<T> {
    fn from(error: CustomErrors) -> ResponseBody<T> {
        ResponseBody {
            succsess: false,
            data: None,
            error: Some(error.into()),
        }
    }
}


impl<T> IntoResponse for ResponseBody<T> {
    fn into_response(self) -> Response {
        let response = (self.error.and_then(|item| item.status), self);

        // match self {
        //     CustomErrors::DieselError { error, message } => (
        //         StatusCode::BAD_REQUEST,
        //         Json(json!({"error":error.to_string(), "extra":message})),
        //     ),
        //     CustomErrors::Argon2Error {
        //         status,
        //         error,
        //         message,
        //     } => (
        //         status,
        //         Json(json!({"error":error.to_string(), "extra":message})),
        //     ),
        //     CustomErrors::StringError { status, error } => (status, Json(json!({"error":error}))),
        //     CustomErrors::PoolConnectionError(error) => (
        //         StatusCode::INTERNAL_SERVER_ERROR,
        //         Json(
        //             json!({"error":error.to_string(), "extra":"Failed to get a database connection"}),
        //         ),
        //     ),
        // };

        response.into_response()
    }
}
