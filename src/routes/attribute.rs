use crate::{
    models::{
        attribute::{
            AttributeWithAttributeValues, NewAttributeWithAttributeValuesName, UpdateAttribute,
        },
        error::CustomErrors,
    },
    pagination::AttributeListPagination,
    services::attribute::{
        create_attributes, get_attributes, multiple_delete_attributes, multiple_update_attributes,
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
    path = "/attribute",
    request_body = [NewAttributeWithAttributeValuesName],
    responses(
        (status = 200, description = "Attributes and their dependences create successfully", body=[AttributeWithAttributeValues]),
        (status = 401, description = "Unauthorized to create Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
pub async fn attribute_create(
    State(state): State<AppState>,

    Json(attribute_info): Json<Vec<NewAttributeWithAttributeValuesName>>,
) -> HandlerResult<Vec<AttributeWithAttributeValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_attributes(&mut connection, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/attribute",
    responses(
        (status = 200, description = "List matching Attributes and their dependences by query", body=[AttributeWithAttributeValues]),
        (status = 401, description = "Unauthorized to list Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AttributeListPagination
    )
)]
pub async fn attribute_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeListPagination>,
) -> HandlerResult<Vec<AttributeWithAttributeValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as AttributeListPagination;
    match get_attributes(&mut connection, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/attribute/multiple_delete",
    request_body = [i32],
    responses(
        (status = 200, description = "Attributes and their dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    )
)]
pub async fn attribute_multiple_delete(
    State(state): State<AppState>,

    Json(attribute_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_attributes(&mut connection, attribute_info).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/attribute/multiple_update",
    request_body = [UpdateAttribute],
    responses(
        (status = 200, description = "Attributes and their dependences updated successfully", body=[AttributeWithAttributeValues]),
        (status = 401, description = "Unauthorized to update Attributes and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Attributes and their dependences not found")
    )
)]
pub async fn attribute_multiple_update(
    State(state): State<AppState>,

    Json(attribute_info): Json<Vec<UpdateAttribute>>,
) -> HandlerResult<Vec<AttributeWithAttributeValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_attributes(&mut connection, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn attribute_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_create).get(attribute_list))
        .route("/multiple_delete", delete(attribute_multiple_delete))
        .route("/multiple_patch", patch(attribute_multiple_update))
}
