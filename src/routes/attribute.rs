use crate::{
    entity::attributes::{NewAttributeWithAttributeValuesModel, UpdateAttributeModel},
    models::error::CustomErrors,
    pagination::AttributeListPagination,
    services::attribute::{
        create_attributes, get_attributes, multiple_delete_attributes, multiple_update_attributes,
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
    path = "/attributes",
    context_path ="/api/v1",
    request_body = [NewAttributeWithAttributeValuesModel],
    responses(
        (status = 200, description = "Attributes and their dependences create successfully", body = [AttributeWithAttributeValuesModel]),
        (status = 401, description = "Unauthorized to create Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_create(
    State(state): State<AppState>,
    Json(attribute_info): Json<Vec<NewAttributeWithAttributeValuesModel>>,
) -> impl IntoResponse {
    match create_attributes(&state.db_sea, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/attributes",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Attributes and their dependences by query", body = [AttributeWithAttributeValuesModel]),
        (status = 401, description = "Unauthorized to list Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AttributeListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeListPagination>,
) -> impl IntoResponse {
    match get_attributes(&state.db_sea, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/attributes/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Attributes and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_attributes(&state.db_sea, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/attributes/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAttributeModel],
    responses(
        (status = 200, description = "Attributes and their dependences updated successfully", body = [AttributeWithAttributeValuesModel]),
        (status = 401, description = "Unauthorized to update Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Attributes and their dependences not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_multiple_update(
    State(state): State<AppState>,
    Json(attribute_info): Json<Vec<UpdateAttributeModel>>,
) -> impl IntoResponse {
    match multiple_update_attributes(&state.db_sea, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn attribute_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_create).get(attribute_list))
        .route("/multiple_delete", delete(attribute_multiple_delete))
        .route("/multiple_patch", patch(attribute_multiple_update))
}
