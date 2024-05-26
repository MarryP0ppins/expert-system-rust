use crate::{
    entity::rule_question_answer::Model as RuleQuestionAnswerModel,
    models::error::CustomErrors,
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
    request_body = [RuleQuestionAnswerModel],
    responses(
        (status = 200, description = "RuleQuestionAnswers and their dependences create successfully", body = [RuleQuestionAnswerModel]),
        (status = 401, description = "Unauthorized to create RuleQuestionAnswers and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_question_answer_create(
    State(state): State<AppState>,
    Json(rule_question_answer_info): Json<Vec<RuleQuestionAnswerModel>>,
) -> impl IntoResponse {
    match create_rule_question_answers(&state.db_sea, rule_question_answer_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "RuleQuestionAnswers and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete RuleQuestionAnswers and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "RuleQuestionAnswers not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn rule_question_answer_multiple_delete(
    State(state): State<AppState>,
    Json(rule_question_answer_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_rule_question_answers(&state.db_sea, rule_question_answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
