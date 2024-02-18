use crate::{
    models::{error::CustomErrors, rule_attributevalue::NewRuleAttributeValue},
    services::rule_attributevalue::{
        create_rule_attributevalues, multiple_delete_rule_attributevalues,
    },
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{extract::State, routing::post, Json, Router};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[debug_handler]
pub async fn rule_attributevalue_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(rule_attributevalue_info): Json<Vec<NewRuleAttributeValue>>,
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

    match create_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => Ok(json!({"created":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
pub async fn rule_attributevalue_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(rule_attributevalue_info): Json<Vec<i32>>,
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

    match multiple_delete_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub fn rule_attributevalue_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_attributevalue_create))
        .route(
            "/multiple_delete",
            post(rule_attributevalue_multiple_delete),
        )
}
