use crate::{
    error::CustomErrors,
    pagination::ObjectListPagination,
    services::object::{
        create_objects, get_objects, multiple_delete_objects, multiple_update_objects,
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
use entity::objects::{NewObjectWithAttributesValueIdsModel, UpdateObjectModel};

#[utoipa::path(
    post,
    path = "/object",
    context_path ="/api/v1",
    request_body = [NewObjectWithAttributesValueIdsModel],
    responses(
        (status = 200, description = "Objects and their dependences create successfully", body = [ObjectWithAttributesValuesModel]),
        (status = 401, description = "Unauthorized to create Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn object_create(
    State(state): State<AppState>,
    Json(object_info): Json<Vec<NewObjectWithAttributesValueIdsModel>>,
) -> impl IntoResponse {
    match create_objects(&state.db_sea, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/object",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Objects and their dependences by query", body = [ObjectWithAttributesValuesModel]),
        (status = 401, description = "Unauthorized to list Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        ObjectListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn object_list(
    State(state): State<AppState>,
    Query(pagination): Query<ObjectListPagination>,
) -> impl IntoResponse {
    match get_objects(&state.db_sea, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/object/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Objects and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Objects not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn object_multiple_delete(
    State(state): State<AppState>,
    Json(object_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_objects(&state.db_sea, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/object/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateObjectModel],
    responses(
        (status = 200, description = "Objects and their dependences updated successfully", body = [ObjectWithAttributesValuesModel]),
        (status = 401, description = "Unauthorized to update Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Objects and their dependences not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn object_multiple_update(
    State(state): State<AppState>,
    Json(object_info): Json<Vec<UpdateObjectModel>>,
) -> impl IntoResponse {
    match multiple_update_objects(&state.db_sea, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn object_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(object_create).get(object_list))
        .route("/multiple_delete", delete(object_multiple_delete))
        .route("/multiple_patch", patch(object_multiple_update))
}
