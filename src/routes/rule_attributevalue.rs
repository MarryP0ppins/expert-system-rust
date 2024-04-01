use crate::{
    models::{error::CustomErrors, rule_attributevalue::NewRuleAttributeValue},
    services::rule_attributevalue::{
        create_rule_attributevalues, multiple_delete_rule_attributevalues,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/rule-attributevalues",
    context_path ="/api/v1",
    request_body = [NewRuleAttributeValue],
    responses(
        (status = 200, description = "RuleAttributeValues and their dependences create successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to create RuleAttributeValue and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn rule_attributevalue_create(
    State(state): State<AppState>,

    Json(rule_attributevalue_info): Json<Vec<NewRuleAttributeValue>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => Ok(()),
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
        (status = 200, description = "RuleAttributeValues and their dependences deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete RuleAttributeValues and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "RuleAttributeValues not found")
    )
)]
#[debug_handler]
pub async fn rule_attributevalue_multiple_delete(
    State(state): State<AppState>,
    Json(rule_attributevalue_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => Ok(()),
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
