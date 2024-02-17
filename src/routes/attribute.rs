use crate::{
    models::{
        attribute::{
            AttributeWithAttributeValues, NewAttributeWithAttributeValuesName, UpdateAttribute,
        },
        error::CustomErrors,
    },
    services::attribute::{
        create_attributes, get_attributes, multiple_delete_attributes, multiple_update_attributes,
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

pub async fn attribute_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_info): Json<Vec<NewAttributeWithAttributeValuesName>>,
) -> HandlerResult<Vec<AttributeWithAttributeValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_attributes(&mut connection, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_list(
    State(state): State<AppState>,
    Query(system): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<AttributeWithAttributeValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_attributes(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_info): Json<Vec<i32>>,
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

    match multiple_delete_attributes(&mut connection, attribute_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_info): Json<Vec<UpdateAttribute>>,
) -> HandlerResult<Vec<AttributeWithAttributeValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_update_attributes(&mut connection, attribute_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub fn attribute_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_create).get(attribute_list))
        .route("/multiple_delete", post(attribute_multiple_delete))
        .route("/multiple_patch", post(attribute_multiple_update))
}
