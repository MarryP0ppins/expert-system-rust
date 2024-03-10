use crate::{
    models::{
        clause::{Clause, NewClause, UpdateClause},
        error::CustomErrors,
    },
    pagination::ClauseListPagination,
    services::clause::{
        create_clauses, get_clauses, multiple_delete_clauses, multiple_update_clauses,
    },
    AppState, HandlerResult,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{delete, patch, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};

#[utoipa::path(
    post,
    path = "/clause",
    context_path ="/api/v1",
    request_body = [NewClause],
    responses(
        (status = 200, description = "Clauses create successfully", body=[Clause]),
        (status = 401, description = "Unauthorized to create Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
pub async fn clause_create(
    State(state): State<AppState>,

    Json(clause_info): Json<Vec<NewClause>>,
) -> HandlerResult<Vec<Clause>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_clauses(&mut connection, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Clauses by query", body=[Clause]),
        (status = 401, description = "Unauthorized to list Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        ClauseListPagination
    )
)]
pub async fn clause_list(
    State(state): State<AppState>,
    Query(pagination): Query<ClauseListPagination>,
) -> HandlerResult<Vec<Clause>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as ClauseListPagination;

    match get_clauses(&mut connection, pagination.rule_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "Clauses deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Clauses not found")
    )
)]
pub async fn clause_multiple_delete(
    State(state): State<AppState>,

    Json(clause_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_clauses(&mut connection, clause_info).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/clause/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateClause],
    responses(
        (status = 200, description = "Clauses updated successfully", body=[Clause]),
        (status = 401, description = "Unauthorized to update Clauses", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Clauses not found")
    )
)]
pub async fn clause_multiple_update(
    State(state): State<AppState>,

    Json(clause_info): Json<Vec<UpdateClause>>,
) -> HandlerResult<Vec<Clause>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_clauses(&mut connection, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
