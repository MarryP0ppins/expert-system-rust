use crate::{
    models::{error::CustomErrors, rule_attributevalue::NewRuleAttributeValue},
    services::rule_attributevalue::{
        create_rule_attributevalues, multiple_delete_rule_attributevalues,
    },
    AppState, HandlerResult,
};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};

#[utoipa::path(
    post,
    path = "/rule-attributevalues",
    context_path ="/api/v1",
    request_body = [NewRuleAttributeValue],
    responses(
        (status = 200, description = "RuleAttributeValues and their dependences create successfully", body = Value, example = json!({"created":"successful"})),
        (status = 401, description = "Unauthorized to create RuleAttributeValue and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
pub async fn rule_attributevalue_create(
    State(state): State<AppState>,

    Json(rule_attributevalue_info): Json<Vec<NewRuleAttributeValue>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => Ok(Json(json!({"created":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/rule-attributevalues/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "RuleAttributeValues and their dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete RuleAttributeValues and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "RuleAttributeValues not found")
    )
)]
pub async fn rule_attributevalue_multiple_delete(
    State(state): State<AppState>,

    Json(rule_attributevalue_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn rule_attributevalue_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_attributevalue_create))
        .route(
            "/multiple_delete",
            delete(rule_attributevalue_multiple_delete),
        )
}
