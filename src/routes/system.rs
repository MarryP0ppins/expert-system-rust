use crate::{
    models::{
        error::CustomErrors,
        system::{NewSystem, System, UpdateSystem},
    },
    pagination::SystemListPagination,
    services::system::{create_system, delete_system, get_system, get_systems, update_system},
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[debug_handler]
pub async fn system_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(system_info): Json<NewSystem>,
) -> HandlerResult<System> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_system(&mut connection, system_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn system_list(
    State(state): State<AppState>,
    Query(pagination): Query<SystemListPagination>,
) -> HandlerResult<Vec<System>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    let pagination: SystemListPagination = pagination;
    match get_systems(
        &mut connection,
        pagination.name.as_deref(),
        pagination.user_id,
    )
    .await
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn system_retrieve(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> HandlerResult<System> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match get_system(&mut connection, system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn system_partial_update(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
    cookie: Cookies,
    system_info: Json<UpdateSystem>,
) -> HandlerResult<System> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match update_system(&mut connection, system_id, system_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn system_delete(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
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

    match delete_system(&mut connection, system_id).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

pub fn system_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(system_create).get(system_list))
        .route(
            "/:system_id",
            get(system_retrieve)
                .patch(system_partial_update)
                .delete(system_delete),
        )
}
