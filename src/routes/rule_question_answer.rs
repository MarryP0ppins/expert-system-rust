use crate::{
    models::{error::CustomErrors, rule_question_answer::NewRuleQuestionAnswer},
    services::rule_question_answer::{
        create_rule_question_answers, multiple_delete_rule_question_answers,
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
    path = "/rule-answer",
    context_path ="/api/v1",
    request_body = [NewRuleQuestionAnswer],
    responses(
        (status = 200, description = "RuleQuestionAnswers and their dependences create successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to create RuleQuestionAnswers and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn rule_question_answer_create(
    State(state): State<AppState>,
    Json(rule_question_answer_info): Json<Vec<NewRuleQuestionAnswer>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match create_rule_question_answers(&mut connection, rule_question_answer_info).await {
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
        (status = 200, description = "RuleQuestionAnswers and their dependences deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete RuleQuestionAnswers and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "RuleQuestionAnswers not found")
    )
)]
#[debug_handler]
pub async fn rule_question_answer_multiple_delete(
    State(state): State<AppState>,
    Json(rule_question_answer_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_delete_rule_question_answers(&mut connection, rule_question_answer_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn rule_question_answer_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_question_answer_create))
        .route(
            "/multiple_delete",
            delete(rule_question_answer_multiple_delete),
        )
}
