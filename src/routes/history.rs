use crate::{
    models::{
        error::CustomErrors,
        history::{HistoryWithSystemAndUser, NewHistory},
    },
    pagination::HistoryListPagination,
    services::history::{create_history, delete_history, get_histories},
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[debug_handler]
pub async fn history_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(history_info): Json<NewHistory>,
) -> HandlerResult<HistoryWithSystemAndUser> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_history(&mut connection, history_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn history_list(
    State(state): State<AppState>,
    Query(pagination): Query<HistoryListPagination>,
    cookie: Cookies,
) -> HandlerResult<Vec<HistoryWithSystemAndUser>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    let pagination: HistoryListPagination = pagination;

    match get_histories(&mut connection, pagination.system, pagination.user).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn history_delete(
    State(state): State<AppState>,
    Path(history_id): Path<i32>,
    cookie: Cookies,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match delete_history(&mut connection, history_id).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

pub fn history_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(history_create).get(history_list))
        .route("/:system_id", delete(history_delete))
}
