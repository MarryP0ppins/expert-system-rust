use crate::{
    models::{
        error::CustomErrors,
        history::NewHistory,
        response_body::{ResponseBodyEmpty, ResponseBodyHistories, ResponseBodyHistory},
    },
    pagination::HistoryListPagination,
    services::history::{create_history, delete_history, get_histories},
    AppState,
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/histories",
    context_path ="/api/v1",
    request_body = NewHistory,
    responses(
        (status = 200, description = "Histories create successfully", body = ResponseBodyHistory),
        (status = 401, description = "Unauthorized to create Histories", body = ResponseBodyHistory, example = json!(ResponseBodyHistory::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn history_create(
    State(state): State<AppState>,
    Json(history_info): Json<NewHistory>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyHistory::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_history(&mut connection, history_info).await {
        Ok(result) => ResponseBodyHistory::from(result),
        Err(err) => ResponseBodyHistory::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/histories",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Histories by query", body = ResponseBodyHistories),
        (status = 401, description = "Unauthorized to list Histories", body = ResponseBodyHistories, example = json!(ResponseBodyHistories::unauthorized_example()))
    ),
    params(
        HistoryListPagination
    )
)]
#[debug_handler]
pub async fn history_list(
    State(state): State<AppState>,
    Query(pagination): Query<HistoryListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyHistories::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination: HistoryListPagination = pagination;

    match get_histories(&mut connection, pagination.system, pagination.user).await {
        Ok(result) => ResponseBodyHistories::from(result),
        Err(err) => ResponseBodyHistories::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/histories/{id}",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "History deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete History", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "History not found")
    ),
    params(
        ("id" = i32, Path, description = "History database id")
    ),
)]
#[debug_handler]
pub async fn history_delete(
    State(state): State<AppState>,
    Path(history_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match delete_history(&mut connection, history_id).await {
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

pub fn history_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(history_create).get(history_list))
        .route("/:system_id", delete(history_delete))
}
