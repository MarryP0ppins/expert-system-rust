use crate::{
    models::{
        attribute_rule_group::{
            AttributeRuleGroupWithRulesAndAttributesValues,
            NewAttributeRuleGroupWithRulesAndAttributesValues,
        },
        error::CustomErrors,
    },
    services::attribute_rule_group::{
        create_attribute_rule_groups, get_attribute_rule_groups,
        multiple_delete_attribute_rule_groups,
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

pub async fn attribute_rule_group_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_rule_group_info): Json<Vec<NewAttributeRuleGroupWithRulesAndAttributesValues>>,
) -> HandlerResult<Vec<AttributeRuleGroupWithRulesAndAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_attribute_rule_groups(&mut connection, attribute_rule_group_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_rule_group_list(
    State(state): State<AppState>,
    Query(system): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<AttributeRuleGroupWithRulesAndAttributesValues>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_attribute_rule_groups(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub async fn attribute_rule_group_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(attribute_rule_group_info): Json<Vec<i32>>,
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

    match multiple_delete_attribute_rule_groups(&mut connection, attribute_rule_group_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub fn attribute_rule_group_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(attribute_rule_group_create).get(attribute_rule_group_list),
        )
        .route(
            "/multiple_delete",
            post(attribute_rule_group_multiple_delete),
        )
}
