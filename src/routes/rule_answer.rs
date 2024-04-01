use crate::{
    models::{error::CustomErrors, rule_answer::NewRuleAnswer},
    services::rule_answer::{create_rule_answers, multiple_delete_rule_answers},
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
    path = "/rule-answer",
    context_path ="/api/v1",
    request_body = [NewRuleAnswer],
    responses(
        (status = 200, description = "RuleAnswers and their dependences create successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to create RuleAnswers and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn rule_answer_create(
    State(state): State<AppState>,
    Json(rule_answer_info): Json<Vec<NewRuleAnswer>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_rule_answers(&mut connection, rule_answer_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/rule-answer/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "RuleAnswers and their dependences deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete RuleAnswers and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "RuleAnswers not found")
    )
)]
#[debug_handler]
pub async fn rule_answer_multiple_delete(
    State(state): State<AppState>,
    Json(rule_answer_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_rule_answers(&mut connection, rule_answer_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn rule_answer_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_answer_create))
        .route("/multiple_delete", delete(rule_answer_multiple_delete))
}
