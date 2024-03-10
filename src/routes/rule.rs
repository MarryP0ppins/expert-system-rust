use crate::{
    models::{
        error::CustomErrors,
        response_body::{ResponseBodyEmpty, ResponseBodyRule, ResponseBodyRules},
        rule::NewRule,
    },
    pagination::RuleListPagination,
    services::rule::{create_rule, get_rules, multiple_delete_rules},
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/rule",
    context_path ="/api/v1",
    request_body = [NewRule],
    responses(
        (status = 200, description = "Rule create successfully", body = ResponseBodyRule),
        (status = 401, description = "Unauthorized to create Rule", body = ResponseBodyRule, example = json!(ResponseBodyRule::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn rule_create(
    State(state): State<AppState>,
    Json(rule_info): Json<NewRule>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyRule::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_rule(&mut connection, rule_info).await {
        Ok(result) => ResponseBodyRule::from(result),
        Err(err) => ResponseBodyRule::from(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Rules and their dependences by query", body = ResponseBodyRules),
        (status = 401, description = "Unauthorized to list Rules and their dependences", body = ResponseBodyRules, example = json!(ResponseBodyRules::unauthorized_example()))
    ),
    params(
        RuleListPagination
    )
)]
#[debug_handler]
pub async fn rule_list(
    State(state): State<AppState>,
    Query(pagination): Query<RuleListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyRules::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as RuleListPagination;

    match get_rules(&mut connection, pagination.system_id).await {
        Ok(result) => ResponseBodyRules::from(result),
        Err(err) => ResponseBodyRules::from(CustomErrors::DieselError {
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
        (status = 200, description = "Rules and their dependences deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete Rules and their dependences", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "Rules not found")
    )
)]
#[debug_handler]
pub async fn rule_multiple_delete(
    State(state): State<AppState>,
    Json(rule_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_rules(&mut connection, rule_info).await {
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

pub fn rule_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_create).get(rule_list))
        .route("/multiple_delete", delete(rule_multiple_delete))
}
