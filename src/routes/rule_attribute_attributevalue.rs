use crate::{
    models::{error::CustomErrors, rule_attribute_attributevalue::NewRuleAttributeAttributeValue},
    services::rule_attribute_attributevalue::{
        create_rule_attribute_attributevalues, multiple_delete_rule_attribute_attributevalues,
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

#[utoipa::path(
    post,
    path = "/rule-attributevalues",
    context_path ="/api/v1",
    request_body = [NewRuleAttributeAttributeValue],
    responses(
        (status = 200, description = "RuleAttributeAttributeValues and their dependences create successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to create RuleAttributeAttributeValue and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_attribute_attributevalue_create(
    State(state): State<AppState>,

    Json(rule_attribute_attributevalue_info): Json<Vec<NewRuleAttributeAttributeValue>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match create_rule_attribute_attributevalues(&mut connection, rule_attribute_attributevalue_info)
        .await
    {
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
        (status = 200, description = "RuleAttributeAttributeValues and their dependences deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete RuleAttributeAttributeValues and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "RuleAttributeAttributeValues not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_attribute_attributevalue_multiple_delete(
    State(state): State<AppState>,
    Json(rule_attribute_attributevalue_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_delete_rule_attribute_attributevalues(
        &mut connection,
        rule_attribute_attributevalue_info,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn rule_attribute_attributevalue_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_attribute_attributevalue_create))
        .route(
            "/multiple_delete",
            delete(rule_attribute_attributevalue_multiple_delete),
        )
}
