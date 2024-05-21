use crate::{
    models::{
        attribute_value::{NewAttributeValue, UpdateAttributeValue},
        error::CustomErrors,
    },
    pagination::AttributeValueListPagination,
    services::attribute_value::{
        create_attributes_values, get_attribute_values, multiple_delete_attributes_values,
        multiple_update_attributes_values,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};

#[utoipa::path(
    post,
    path = "/attributevalues",
    context_path ="/api/v1",
    request_body = [NewAttributeValue],
    responses(
        (status = 200, description = "AttributeValues create successfully", body = [AttributeValue]),
        (status = 401, description = "Unauthorized to create AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn attribute_value_create(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<NewAttributeValue>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match create_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/attributevalues",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching AttributeValues by query", body = [AttributeValue]),
        (status = 401, description = "Unauthorized to list AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AttributeValueListPagination
    )
)]
#[debug_handler]
pub async fn attribute_value_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeValueListPagination>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    let pagination = pagination as AttributeValueListPagination;
    match get_attribute_values(&mut connection, pagination.attribute_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/attributevalues/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "AttributeValues deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "AttributeValues not found")
    )
)]
#[debug_handler]
pub async fn attribute_value_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_delete_attributes_values(&mut connection, attribute_value_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/attributevalues/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAttributeValue],
    responses(
        (status = 200, description = "AttributeValues updated successfully", body = [AttributeValue]),
        (status = 401, description = "Unauthorized to update AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "AttributeValues not found")
    )
)]
pub async fn attribute_value_multiple_update(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<UpdateAttributeValue>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_update_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn attribute_value_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_value_create).get(attribute_value_list))
        .route("/multiple_delete", delete(attribute_value_multiple_delete))
        .route("/multiple_patch", patch(attribute_value_multiple_update))
}
