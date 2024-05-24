use crate::{
    models::{
        error::CustomErrors,
        system::{NewSystemMultipart, SystemDelete, UpdateSystemMultipart},
    },
    pagination::SystemListPagination,
    services::{
        backup::{backup_from_system, system_from_backup},
        system::{
            create_system, delete_system, get_ready_to_start_system, get_system, get_systems,
            update_system,
        },
    },
    utils::auth::{cookie_check, password_check},
    AppState,
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_typed_multipart::TypedMultipart;
use tower_cookies::Cookies;

#[utoipa::path(
    post,
    path = "/systems",
    context_path ="/api/v1",
    request_body(content = NewSystemMultipart, description = "Multipart file", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "System create successfully", body=System),
        (status = 401, description = "Unauthorized to create System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn system_create(
    State(state): State<AppState>,
    cookie: Cookies,
    TypedMultipart(system_info): TypedMultipart<NewSystemMultipart>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    let user = cookie_check(&mut connection, cookie, &state.cookie_key).await?;

    match create_system(&mut connection, system_info, user.id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Systems by query", body = [TSystem]),
        (status = 401, description = "Unauthorized to list Systems", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        SystemListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_list(
    State(state): State<AppState>,
    Query(pagination): Query<SystemListPagination>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    let pagination: SystemListPagination = pagination;
    match get_systems(&mut connection, pagination).await {
        Ok(result) => {
            let mut headers = HeaderMap::new();
            headers.insert("x-pages", result.pages.to_string().parse().unwrap());
            Ok((headers, Json(result.systems)))
        }
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
        ("id" = u32, Path, description = "System database id")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_retrieve(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

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
    path = "/systems/{id}/test",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching System by query", body=SystemData),
        (status = 401, description = "Unauthorized to retrive System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_start(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match get_ready_to_start_system(&mut connection, system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/systems/{id}/backup",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching System by query", body=Vec<u8>),
        (status = 401, description = "Unauthorized to retrive System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_backup(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match backup_from_system(&mut connection, system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/systems/restore",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Sususfully restore", body = System),
        (status = 401, description = "Unauthorized to retrive System", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_restore(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(system_decode): Json<Vec<u8>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match system_from_backup(&mut connection, system_decode, cookie, &state.cookie_key).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err),
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
        ("id" = u32, Path, description = "System database id")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_partial_update(
    State(state): State<AppState>,
    Path(system_id): Path<i32>,
    TypedMultipart(system_info): TypedMultipart<UpdateSystemMultipart>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

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
    request_body=SystemDelete,
    responses(
        (status = 200, description = "System and it dependences deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete System and it dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "System not found")
    ),
    params(
        ("id" = u32, Path, description = "System database id")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn system_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Path(system_id): Path<i32>,
    Json(system_info): Json<SystemDelete>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    password_check(
        &mut connection,
        cookie,
        &state.cookie_key,
        &system_info.password,
    )
    .await?;

    match delete_system(&mut connection, system_id).await {
        Ok(_) => Ok(()),
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
        .route("/:system_id/test", get(system_start))
        .route("/:system_id/backup", get(system_backup))
        .route("/restore", post(system_restore))
}
