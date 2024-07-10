use crate::{
    error::CustomErrors,
    pagination::LikeListPagination,
    services::likes::{create_like, delete_like, get_likes},
    AppState,
};

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use entity::likes::LikesModel;

#[utoipa::path(
    post,
    path = "/likes",
    context_path ="/api/v1",
    request_body = LikesModel,
    responses(
        (status = 200, description = "Like and their dependences create successfully", body = LikesModel),
        (status = 401, description = "Unauthorized to create Like and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn like_create(
    State(state): State<AppState>,
    Json(like_info): Json<LikesModel>,
) -> impl IntoResponse {
    match create_like(&state.db_sea, like_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/likes",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching LIkes by query", body = [LikesModel]),
        (status = 401, description = "Unauthorized to list Answers", body = CustomErrors,
            example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        LikeListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn like_list(
    State(state): State<AppState>,
    Query(pagination): Query<LikeListPagination>,
) -> impl IntoResponse {
    match get_likes(&state.db_sea, pagination.user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/likes/{like_id}",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Like and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Like and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Like not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn like_delete(
    State(state): State<AppState>,
    Path(like_id): Path<i32>,
) -> impl IntoResponse {
    match delete_like(&state.db_sea, like_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn like_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(like_create).get(like_list))
        .route("/:like_id", delete(like_delete))
}
