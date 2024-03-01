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

#[utoipa::path(
    post,
    path = "/history",
    request_body = NewHistory,
    responses(
        (status = 200, description = "Histories create successfully", body=HistoryWithSystemAndUser),
        (status = 401, description = "Unauthorized to create Histories", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
pub async fn history_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(history_info): Json<NewHistory>,
) -> HandlerResult<HistoryWithSystemAndUser> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match create_history(&mut connection, history_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/history",
    responses(
        (status = 200, description = "List matching Histories by query", body=[HistoryWithSystemAndUser]),
        (status = 401, description = "Unauthorized to list Histories", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        HistoryListPagination
    )
)]
pub async fn history_list(
    State(state): State<AppState>,
    Query(pagination): Query<HistoryListPagination>,
    cookie: Cookies,
) -> HandlerResult<Vec<HistoryWithSystemAndUser>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    let pagination: HistoryListPagination = pagination;

    match get_histories(&mut connection, pagination.system, pagination.user).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/history/{id}",
    responses(
        (status = 200, description = "History deleted successfully"),
        (status = 401, description = "Unauthorized to delete History", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "History not found")
    ),
    params(
        ("id" = i32, Path, description = "History database id")
    ),
)]
pub async fn history_delete(
    State(state): State<AppState>,
    Path(history_id): Path<i32>,
    cookie: Cookies,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match delete_history(&mut connection, history_id).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
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
