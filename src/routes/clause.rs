use crate::{
    entity::clauses::{Model as ClauseModel, UpdateClauseModel},
    models::error::CustomErrors,
    pagination::ClauseListPagination,
    services::clause::{
        create_clauses, get_clauses, multiple_delete_clauses, multiple_update_clauses,
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
    path = "/clause",
    context_path ="/api/v1",
    request_body = [ClauseModel],
    responses(
        (status = 200, description = "Clauses create successfully", body = [ClauseModel]),
        (status = 401, description = "Unauthorized to create Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn clause_create(
    State(state): State<AppState>,
    Json(clause_info): Json<Vec<ClauseModel>>,
) -> impl IntoResponse {
    match create_clauses(&state.db_sea, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/clause",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Clauses by query", body = [ClauseModel]),
        (status = 401, description = "Unauthorized to list Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        ClauseListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn clause_list(
    State(state): State<AppState>,
    Query(pagination): Query<ClauseListPagination>,
) -> impl IntoResponse {
    match get_clauses(&state.db_sea, pagination.rule_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/clause/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Clauses deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Clauses not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn clause_multiple_delete(
    State(state): State<AppState>,
    Json(clause_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_clauses(&state.db_sea, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/clause/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateClauseModel],
    responses(
        (status = 200, description = "Clauses updated successfully", body = [ClauseModel]),
        (status = 401, description = "Unauthorized to update Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Clauses not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn clause_multiple_update(
    State(state): State<AppState>,
    Json(clause_info): Json<Vec<UpdateClauseModel>>,
) -> impl IntoResponse {
    match multiple_update_clauses(&state.db_sea, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn clause_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(clause_create).get(clause_list))
        .route("/multiple_delete", delete(clause_multiple_delete))
        .route("/multiple_patch", patch(clause_multiple_update))
}
