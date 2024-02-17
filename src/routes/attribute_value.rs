use crate::{
    models::{
        attribute_value::{AttributeValue, NewAttributeValue, UpdateAttributeValue},
        error::CustomErrors,
    },
    services::attribute_value::{
        create_attributes_values, get_attribute_values, multiple_delete_attributes_values,
        multiple_update_attributes_values,
    },
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{
    extract::{Query, State},
    routing::post,
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub async fn attribute_value_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_value_info): Json<Vec<NewAttributeValue>>,
) -> HandlerResult<Vec<AttributeValue>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_value_list(
    State(state): State<AppState>,
    Query(attribute): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<AttributeValue>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_attribute_values(&mut connection, attribute).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_value_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_value_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_delete_attributes_values(&mut connection, attribute_value_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_value_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_value_info): Json<Vec<UpdateAttributeValue>>,
) -> HandlerResult<Vec<AttributeValue>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_update_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
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
