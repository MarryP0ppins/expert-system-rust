use crate::{
    models::{
        error::CustomErrors,
        system::{NewSystemMultipart, System, UpdateSystemMultipart},
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
use axum_macros::debug_handler;
use axum_typed_multipart::TypedMultipart;
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[utoipa::path(
    post,
    path = "/system",
    request_body(content = NewSystemMultipart, description = "Multipart file", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "System create successfully", body=System),
        (status = 401, description = "Unauthorized to create System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    )
)]
pub async fn system_create(
    State(state): State<AppState>,
    cookie: Cookies,
    TypedMultipart(system_info): TypedMultipart<NewSystemMultipart>,
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
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    get,
    path = "/system",
    responses(
        (status = 200, description = "List matching Systems by query", body=[System]),
        (status = 401, description = "Unauthorized to list Systems", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    ),
    params(
        SystemListPagination
    )
)]
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
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    get,
    path = "/system/{id}",
    responses(
        (status = 200, description = "Matching System by query", body=System),
        (status = 401, description = "Unauthorized to retrive System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    ),
    params(
        ("id" = i32, Path, description = "System database id")
    ),
)]
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
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
#[utoipa::path(
    patch,
    path = "/system/{id}",
    request_body(content = UpdateSystemMultipart, description = "Multipart file", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "System and it dependences updated successfully", body = System),
        (status = 401, description = "Unauthorized to update System and it dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        })),
        (status = 404, description = "System not found")
    ),
    params(
        ("id" = i32, Path, description = "System database id")
    ),
)]
pub async fn system_partial_update(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
    cookie: Cookies,
    TypedMultipart(system_info): TypedMultipart<UpdateSystemMultipart>,
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

    match update_system(&mut connection, system_id, system_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/{id}",
    responses(
        (status = 200, description = "System and it dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete System and it dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        })),
        (status = 404, description = "System not found")
    ),
    params(
        ("id" = i32, Path, description = "System database id")
    ),
)]
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
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
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
