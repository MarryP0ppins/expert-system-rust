use crate::{
    models::{
        clause::{NewClause, UpdateClause},
        error::CustomErrors,
        response_body::{ResponseBodyClauses, ResponseBodyEmpty},
    },
    pagination::ClauseListPagination,
    services::clause::{
        create_clauses, get_clauses, multiple_delete_clauses, multiple_update_clauses,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/clause",
    context_path ="/api/v1",
    request_body = [NewClause],
    responses(
        (status = 200, description = "Clauses create successfully", body = ResponseBodyClauses),
        (status = 401, description = "Unauthorized to create Clauses", body = ResponseBodyClauses, example = json!(ResponseBodyClauses::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn clause_create(
    State(state): State<AppState>,
    Json(clause_info): Json<Vec<NewClause>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyClauses::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_clauses(&mut connection, clause_info).await {
        Ok(result) => ResponseBodyClauses::from(result),
        Err(err) => ResponseBodyClauses::from(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Clauses by query", body = ResponseBodyClauses),
        (status = 401, description = "Unauthorized to list Clauses", body = ResponseBodyClauses, example = json!(ResponseBodyClauses::unauthorized_example()))
    ),
    params(
        ClauseListPagination
    )
)]
#[debug_handler]
pub async fn clause_list(
    State(state): State<AppState>,
    Query(pagination): Query<ClauseListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyClauses::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as ClauseListPagination;

    match get_clauses(&mut connection, pagination.rule_id).await {
        Ok(result) => ResponseBodyClauses::from(result),
        Err(err) => ResponseBodyClauses::from(CustomErrors::DieselError {
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
        (status = 200, description = "Clauses deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete Clauses", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "Clauses not found")
    )
)]
#[debug_handler]
pub async fn clause_multiple_delete(
    State(state): State<AppState>,
    Json(clause_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_clauses(&mut connection, clause_info).await {
        Ok(_) => ResponseBodyEmpty {
            succsess: true,
            data: None,
            error: None,
        },
        Err(err) => ResponseBodyEmpty::from(CustomErrors::DieselError {
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
        (status = 200, description = "Clauses updated successfully", body = ResponseBodyClauses),
        (status = 401, description = "Unauthorized to update Clauses", body = ResponseBodyClauses, example = json!(ResponseBodyClauses::unauthorized_example())),
        (status = 404, description = "Clauses not found")
    )
)]
#[debug_handler]
pub async fn clause_multiple_update(
    State(state): State<AppState>,
    Json(clause_info): Json<Vec<UpdateClause>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyClauses::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_clauses(&mut connection, clause_info).await {
        Ok(result) => ResponseBodyClauses::from(result),
        Err(err) => ResponseBodyClauses::from(CustomErrors::DieselError {
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
