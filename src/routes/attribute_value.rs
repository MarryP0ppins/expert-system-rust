use crate::{
    models::{
        attribute_value::{AttributeValue, NewAttributeValue, UpdateAttributeValue},
        error::CustomErrors,
    },
    pagination::AttributeValueListPagination,
    services::attribute_value::{
        create_attributes_values, get_attribute_values, multiple_delete_attributes_values,
        multiple_update_attributes_values,
    },
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[utoipa::path(
    post,
    path = "/attributevalue",
    request_body = [NewAttributeValue],
    responses(
        (status = 200, description = "AttributeValues create successfully", body=[AttributeValue]),
        (status = 401, description = "Unauthorized to create AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    )
)]
pub async fn attribute_value_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_value_info): Json<Vec<NewAttributeValue>>,
) -> HandlerResult<Vec<AttributeValue>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_attributes_values(&mut connection, attribute_value_info).await {
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
    path = "/attributevalue",
    responses(
        (status = 200, description = "List matching AttributeValues by query", body=[AttributeValue]),
        (status = 401, description = "Unauthorized to list AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    ),
    params(
        AttributeValueListPagination
    )
)]
pub async fn attribute_value_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeValueListPagination>,
    cookie: Cookies,
) -> HandlerResult<Vec<AttributeValue>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    let pagination = pagination as AttributeValueListPagination;
    match get_attribute_values(&mut connection, pagination.attribute_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    post,
    path = "/attributevalue/multiple_delete",
    request_body = [i32],
    responses(
        (status = 200, description = "AttributeValues deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        })),
        (status = 404, description = "AttributeValues not found")
    )
)]
pub async fn attribute_value_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_value_info): Json<Vec<i32>>,
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

    match multiple_delete_attributes_values(&mut connection, attribute_value_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    post,
    path = "/attributevalue/multiple_update",
    request_body = [UpdateAttributeValue],
    responses(
        (status = 200, description = "AttributeValues updated successfully", body=[AttributeValue]),
        (status = 401, description = "Unauthorized to update AttributeValues", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        })),
        (status = 404, description = "AttributeValues not found")
    )
)]
pub async fn attribute_value_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_value_info): Json<Vec<UpdateAttributeValue>>,
) -> HandlerResult<Vec<AttributeValue>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_update_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub fn attribute_value_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_value_create).get(attribute_value_list))
        .route("/multiple_delete", post(attribute_value_multiple_delete))
        .route("/multiple_patch", post(attribute_value_multiple_update))
}
