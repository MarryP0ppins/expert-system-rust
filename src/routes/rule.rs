use crate::{
    models::{
        error::CustomErrors,
        rule::{NewRule, RuleWithClausesAndEffects},
    },
    services::rule::{create_rule, get_rules, multiple_delete_rules},
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

#[debug_handler]
pub async fn rule_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(rule_info): Json<NewRule>,
) -> HandlerResult<RuleWithClausesAndEffects> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_rule(&mut connection, rule_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.into(),
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
pub async fn rule_list(
    State(state): State<AppState>,
    Query(system): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<RuleWithClausesAndEffects>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_rules(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.into(),
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
pub async fn rule_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(rule_info): Json<Vec<i32>>,
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

    match multiple_delete_rules(&mut connection, rule_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err.into(),
            message: None,
        }
        .into()),
    }
}

pub fn rule_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_create).get(rule_list))
        .route("/multiple_delete", post(rule_multiple_delete))
}
