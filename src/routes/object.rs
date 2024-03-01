use crate::{
    models::{
        error::CustomErrors,
        object::{NewObjectWithAttributesValueIds, ObjectWithAttributesValues, UpdateObject},
    },
    pagination::ObjectListPagination,
    services::object::{
        create_objects, get_objects, multiple_delete_objects, multiple_update_objects,
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
    path = "/object",
    request_body = [NewObjectWithAttributesValueIds],
    responses(
        (status = 200, description = "Objects and their dependences create successfully", body=[ObjectWithAttributesValues]),
        (status = 401, description = "Unauthorized to create Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string().to_string(),
        }))
    )
)]
pub async fn object_create(
    State(state): State<AppState>,

    Json(object_info): Json<Vec<NewObjectWithAttributesValueIds>>,
) -> HandlerResult<Vec<ObjectWithAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_objects(&mut connection, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/object",
    responses(
        (status = 200, description = "List matching Objects and their dependences by query", body=[ObjectWithAttributesValues]),
        (status = 401, description = "Unauthorized to list Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string().to_string(),
        }))
    ),
    params(
        ObjectListPagination
    )
)]
pub async fn object_list(
    State(state): State<AppState>,
    Query(pagination): Query<ObjectListPagination>,
) -> HandlerResult<Vec<ObjectWithAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as ObjectListPagination;

    match get_objects(&mut connection, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/object/multiple_delete",
    request_body = [i32],
    responses(
        (status = 200, description = "Objects and their dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string().to_string(),
        })),
        (status = 404, description = "Objects not found")
    )
)]
pub async fn object_multiple_delete(
    State(state): State<AppState>,

    Json(object_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_objects(&mut connection, object_info).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/object/multiple_update",
    request_body = [UpdateObject],
    responses(
        (status = 200, description = "Objects and their dependences updated successfully", body=[ObjectWithAttributesValues]),
        (status = 401, description = "Unauthorized to update Objects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string().to_string(),
        })),
        (status = 404, description = "Objects and their dependences not found")
    )
)]
pub async fn object_multiple_update(
    State(state): State<AppState>,

    Json(object_info): Json<Vec<UpdateObject>>,
) -> HandlerResult<Vec<ObjectWithAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_objects(&mut connection, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn object_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(object_create).get(object_list))
        .route("/multiple_delete", delete(object_multiple_delete))
        .route("/multiple_patch", patch(object_multiple_update))
}
