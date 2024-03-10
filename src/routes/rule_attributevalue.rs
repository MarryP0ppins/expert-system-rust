use crate::{
    models::{
        error::CustomErrors, response_body::ResponseBodyEmpty,
        rule_attributevalue::NewRuleAttributeValue,
    },
    services::rule_attributevalue::{
        create_rule_attributevalues, multiple_delete_rule_attributevalues,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::State,
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
        (status = 200, description = "RuleAttributeValues and their dependences create successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to create RuleAttributeValue and their dependences", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example()))
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
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => ResponseBodyEmpty {
            succsess: true,
            data: None,
            error: None,
        },
        Err(err) => ResponseBodyEmpty::from(CustomErrors::DieselError {
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
        (status = 200, description = "RuleAttributeValues and their dependences deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete RuleAttributeValues and their dependences", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
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
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_rule_attributevalues(&mut connection, rule_attributevalue_info).await {
        Ok(_) => ResponseBodyEmpty {
            succsess: true,
            data: None,
            error: None,
        },
        Err(err) => ResponseBodyEmpty::from(CustomErrors::DieselError {
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
