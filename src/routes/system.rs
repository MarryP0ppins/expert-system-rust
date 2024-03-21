use crate::{
    models::{
        error::CustomErrors,
        response_body::{
            ResponseBodyEmpty, ResponseBodyStartSystem, ResponseBodySystem, ResponseBodySystems,
        },
        system::{NewSystemMultipart, UpdateSystemMultipart},
    },
    pagination::SystemListPagination,
    services::system::{
        create_system, delete_system, get_ready_to_start_system, get_system, get_systems,
        update_system,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_typed_multipart::TypedMultipart;
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

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
#[debug_handler]
pub async fn system_create(
    State(state): State<AppState>,
    TypedMultipart(system_info): TypedMultipart<NewSystemMultipart>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodySystem::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_system(&mut connection, system_info).await {
        Ok(result) => ResponseBodySystem::from(result),
        Err(err) => ResponseBodySystem::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/systems",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Systems by query", body=ResponseBodySystems),
        (status = 401, description = "Unauthorized to list Systems", body = ResponseBodySystems, example = json!(ResponseBodySystems::unauthorized_example()))
    ),
    params(
        SystemListPagination
    )
)]
#[debug_handler]
pub async fn system_list(
    State(state): State<AppState>,
    Query(pagination): Query<SystemListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodySystems::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination: SystemListPagination = pagination;
    match get_systems(&mut connection, pagination).await {
        Ok(result) => ResponseBodySystems::from(result),
        Err(err) => ResponseBodySystems::from(CustomErrors::DieselError {
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
        (status = 200, description = "Matching System by query", body=ResponseBodySystem),
        (status = 401, description = "Unauthorized to retrive System", body = ResponseBodySystem, example = json!(ResponseBodySystem::unauthorized_example()))
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
)]
#[debug_handler]
pub async fn system_retrieve(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodySystem::from(CustomErrors::PoolConnectionError(err)),
    };

    match get_system(&mut connection, system_id).await {
        Ok(result) => ResponseBodySystem::from(result),
        Err(err) => ResponseBodySystem::from(CustomErrors::DieselError {
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
        (status = 200, description = "Matching System by query", body=ResponseBodyStartSystem),
        (status = 401, description = "Unauthorized to retrive System", body = ResponseBodyStartSystem, example = json!(ResponseBodyStartSystem::unauthorized_example()))
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
)]
#[debug_handler]
pub async fn system_start(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyStartSystem::from(CustomErrors::PoolConnectionError(err)),
    };

    match get_ready_to_start_system(&mut connection, system_id).await {
        Ok(result) => ResponseBodyStartSystem::from(result),
        Err(err) => ResponseBodyStartSystem::from(CustomErrors::DieselError {
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
        (status = 200, description = "System and it dependences updated successfully", body = ResponseBodySystem),
        (status = 401, description = "Unauthorized to update System and it dependences", body = ResponseBodySystem, example = json!(ResponseBodySystem::unauthorized_example())),
        (status = 404, description = "System not found")
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
)]
#[debug_handler]
pub async fn system_partial_update(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
    TypedMultipart(system_info): TypedMultipart<UpdateSystemMultipart>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodySystem::from(CustomErrors::PoolConnectionError(err)),
    };

    match update_system(&mut connection, system_id, system_info).await {
        Ok(result) => ResponseBodySystem::from(result),
        Err(err) => ResponseBodySystem::from(CustomErrors::DieselError {
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
        (status = 200, description = "System and it dependences deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete System and it dependences", body = ResponseBodyEmpty, example = json!(ResponseBodySystem::unauthorized_example())),
        (status = 404, description = "System not found")
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
)]
#[debug_handler]
pub async fn system_delete(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match delete_system(&mut connection, system_id).await {
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
