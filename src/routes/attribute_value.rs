use crate::{
    entity::{
        attributesvalues::{Model as AttributeValueModel, UpdateAttributeValueModel},
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
    request_body = [AttributeValueModel],
    responses(
        (status = 200, description = "AttributeValues create successfully", body = [AttributeValueModel]),
        (status = 401, description = "Unauthorized to create AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_value_create(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<AttributeValueModel>>,
) -> impl IntoResponse {
    match create_attributes_values(&state.db_sea, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "List matching AttributeValues by query", body = [AttributeValueModel]),
        (status = 401, description = "Unauthorized to list AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AttributeValueListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_value_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeValueListPagination>,
) -> impl IntoResponse {
    match get_attribute_values(&state.db_sea, pagination.attribute_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "AttributeValues deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "AttributeValues not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_value_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_attributes_values(&state.db_sea, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/attributevalues/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAttributeValueModel],
    responses(
        (status = 200, description = "AttributeValues updated successfully", body = [AttributeValueModel]),
        (status = 401, description = "Unauthorized to update AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "AttributeValues not found")
    ),
    security(("Cookie" = []))
)]
pub async fn attribute_value_multiple_update(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<UpdateAttributeValueModel>>,
) -> impl IntoResponse {
    match multiple_update_attributes_values(&state.db_sea, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
