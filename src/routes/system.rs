use crate::{
    models::{
        error::CustomErrors,
        response_body::ResponseBodySystem,
        system::{NewSystemMultipart, System, SystemData, UpdateSystemMultipart},
    },
    pagination::SystemListPagination,
    services::system::{
        create_system, delete_system, get_ready_to_start_system, get_system, get_systems,
        update_system,
    },
    AppState, HandlerResult,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_typed_multipart::TypedMultipart;
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};

#[utoipa::path(
    post,
    path = "/systems",
    context_path ="/api/v1",
    request_body(content = NewSystemMultipart, description = "Multipart file", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "System create successfully", body=ResponseBodySystem),
        (status = 401, description = "Unauthorized to create System", body = ResponseBodySystem, example = json!(ResponseBodySystem::unauthorized_example()))
    )
)]
pub async fn system_create(
    State(state): State<AppState>,
    TypedMultipart(system_info): TypedMultipart<NewSystemMultipart>,
) -> ResponseBodySystem {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_system(&mut connection, system_info).await {
        Ok(result) => Ok(Json(ResponseBodySystem {
            succsess: true,
            data: Some(result),
            error: None,
        })),
        Err(err) => Err(Json(ResponseBodySystem {
            succsess: false,
            data: None,
            error: Some(CustomErrors::DieselError {
                error: err,
                message: None,
            }),
        })),
    }
}

#[utoipa::path(
    get,
    path = "/systems",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Systems by query", body=[System]),
        (status = 401, description = "Unauthorized to list Systems", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
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
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
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
        }),
    }
}

#[utoipa::path(
    get,
    path = "/systems/{id}",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching System by query", body=System),
        (status = 401, description = "Unauthorized to retrive System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
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
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match get_system(&mut connection, system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/systems/{id}/start",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching System by query", body=SystemData),
        (status = 401, description = "Unauthorized to retrive System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        ("id" = i32, Path, description = "System database id")
    ),
)]
pub async fn system_start(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> HandlerResult<SystemData> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match get_ready_to_start_system(&mut connection, system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/systems/{id}",
    context_path ="/api/v1",
    request_body(content = UpdateSystemMultipart, description = "Multipart file", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "System and it dependences updated successfully", body = System),
        (status = 401, description = "Unauthorized to update System and it dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
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
    TypedMultipart(system_info): TypedMultipart<UpdateSystemMultipart>,
) -> HandlerResult<System> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match update_system(&mut connection, system_id, system_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/systems/{id}",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "System and it dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete System and it dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
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
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match delete_system(&mut connection, system_id).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
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
        .route("/:system_id/start", get(system_start))
}
