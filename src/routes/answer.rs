use crate::{
    models::{
        answer::{Answer, NewAnswer, UpdateAnswer},
        error::CustomErrors,
    },
    services::answer::{
        create_answer, get_answers, multiple_delete_answers, multiple_update_answers,
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

pub async fn answer_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(answer_info): Json<Vec<NewAnswer>>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_answer(&mut connection, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub async fn answer_list(
    State(state): State<AppState>,
    Query(question): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_answers(&mut connection, question).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub async fn answer_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(answer_info): Json<Vec<i32>>,
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

    match multiple_delete_answers(&mut connection, answer_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub async fn answer_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(answer_info): Json<Vec<UpdateAnswer>>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_update_answers(&mut connection, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub fn answer_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(answer_create).get(answer_list))
        .route("/multiple_delete", post(answer_multiple_delete))
        .route("/multiple_patch", post(answer_multiple_update))
}
