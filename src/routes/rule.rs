use crate::{
    entity::{error::CustomErrors, rules::NewRuleWithClausesAndEffects},
    pagination::RuleListPagination,
    services::rule::{create_rule, get_rules, multiple_delete_rules},
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};

#[utoipa::path(
    post,
    path = "/rule",
    context_path ="/api/v1",
    request_body = [NewRuleWithClausesAndEffects],
    responses(
        (status = 200, description = "Rule create successfully", body = [RuleWithClausesAndEffects]),
        (status = 401, description = "Unauthorized to create Rule", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_create(
    State(state): State<AppState>,
    Json(rule_info): Json<Vec<NewRuleWithClausesAndEffects>>,
) -> impl IntoResponse {
    match create_rule(&state.db_sea, rule_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/rule",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Rules and their dependences by query", body = [RuleWithClausesAndEffects]),
        (status = 401, description = "Unauthorized to list Rules and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        RuleListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_list(
    State(state): State<AppState>,
    Query(pagination): Query<RuleListPagination>,
) -> impl IntoResponse {
    match get_rules(&state.db_sea, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/rule/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Rules and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Rules and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Rules not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_multiple_delete(
    State(state): State<AppState>,
    Json(rule_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_rules(&state.db_sea, rule_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn rule_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_create).get(rule_list))
        .route("/multiple_delete", delete(rule_multiple_delete))
}
