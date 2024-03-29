use crate::{
    models::{
        error::CustomErrors,
        object::{NewObjectWithAttributesValueIds, UpdateObject},
        response_body::{ResponseBodyEmpty, ResponseBodyObjects},
    },
    pagination::ObjectListPagination,
    services::object::{
        create_objects, get_objects, multiple_delete_objects, multiple_update_objects,
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
    path = "/object",
    context_path ="/api/v1",
    request_body = [NewObjectWithAttributesValueIds],
    responses(
        (status = 200, description = "Objects and their dependences create successfully", body = ResponseBodyObjects),
        (status = 401, description = "Unauthorized to create Objects and their dependences", body = ResponseBodyObjects, example = json!(ResponseBodyObjects::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn object_create(
    State(state): State<AppState>,
    Json(object_info): Json<Vec<NewObjectWithAttributesValueIds>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyObjects::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_objects(&mut connection, object_info).await {
        Ok(result) => ResponseBodyObjects::from(result),
        Err(err) => ResponseBodyObjects::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/object",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Objects and their dependences by query", body = ResponseBodyObjects),
        (status = 401, description = "Unauthorized to list Objects and their dependences", body = ResponseBodyObjects, example = json!(ResponseBodyObjects::unauthorized_example()))
    ),
    params(
        ObjectListPagination
    )
)]
#[debug_handler]
pub async fn object_list(
    State(state): State<AppState>,
    Query(pagination): Query<ObjectListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyObjects::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as ObjectListPagination;

    match get_objects(&mut connection, pagination.system_id).await {
        Ok(result) => ResponseBodyObjects::from(result),
        Err(err) => ResponseBodyObjects::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/object/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Objects and their dependences deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete Objects and their dependences", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "Objects not found")
    )
)]
#[debug_handler]
pub async fn object_multiple_delete(
    State(state): State<AppState>,
    Json(object_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_objects(&mut connection, object_info).await {
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
    path = "/object/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateObject],
    responses(
        (status = 200, description = "Objects and their dependences updated successfully", body = ResponseBodyObjects),
        (status = 401, description = "Unauthorized to update Objects and their dependences", body = ResponseBodyObjects, example = json!(ResponseBodyObjects::unauthorized_example())),
        (status = 404, description = "Objects and their dependences not found")
    )
)]
#[debug_handler]
pub async fn object_multiple_update(
    State(state): State<AppState>,
    Json(object_info): Json<Vec<UpdateObject>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyObjects::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_objects(&mut connection, object_info).await {
        Ok(result) => ResponseBodyObjects::from(result),
        Err(err) => ResponseBodyObjects::from(CustomErrors::DieselError {
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
