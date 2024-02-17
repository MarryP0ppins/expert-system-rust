use crate::{
    models::{
        error::CustomErrors,
        object::{NewObjectWithAttributesValueIds, ObjectWithAttributesValues, UpdateObject},
    },
    services::object::{
        create_objects, get_objects, multiple_delete_objects, multiple_update_objects,
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

pub async fn object_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(object_info): Json<Vec<NewObjectWithAttributesValueIds>>,
) -> HandlerResult<Vec<ObjectWithAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_objects(&mut connection, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn object_list(
    State(state): State<AppState>,
    Query(system): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<ObjectWithAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_objects(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn object_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(object_info): Json<Vec<i32>>,
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

    match multiple_delete_objects(&mut connection, object_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub async fn object_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(object_info): Json<Vec<UpdateObject>>,
) -> HandlerResult<Vec<ObjectWithAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err.to_string()).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_update_objects(&mut connection, object_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.to_string(),
            message: None,
        }
        .into()),
    }
}

pub fn object_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(object_create).get(object_list))
        .route("/multiple_delete", post(object_multiple_delete))
        .route("/multiple_patch", post(object_multiple_update))
}
