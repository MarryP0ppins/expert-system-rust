use crate::{
    models::{
        error::CustomErrors,
        question::{NewQuestionWithAnswersBody, QuestionWithAnswers, UpdateQuestion},
    },
    pagination::QuestionListPagination,
    services::question::{
        create_questions, get_questions, multiple_delete_questions, multiple_update_questions,
    },
    AppState, HandlerResult,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{delete, patch, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};

#[utoipa::path(
    post,
    path = "/questions",
    context_path ="/api/v1",
    request_body = [NewQuestionWithAnswersBody],
    responses(
        (status = 200, description = "Questions and their dependences create successfully", body=[QuestionWithAnswers]),
        (status = 401, description = "Unauthorized to create Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
pub async fn question_create(
    State(state): State<AppState>,

    Json(question_info): Json<Vec<NewQuestionWithAnswersBody>>,
) -> HandlerResult<Vec<QuestionWithAnswers>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_questions(&mut connection, question_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/questions",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Questions and their dependences by query", body=[QuestionWithAnswers]),
        (status = 401, description = "Unauthorized to list Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        QuestionListPagination
    )
)]
pub async fn question_list(
    State(state): State<AppState>,
    Query(pagination): Query<QuestionListPagination>,
) -> HandlerResult<Vec<QuestionWithAnswers>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as QuestionListPagination;

    match get_questions(&mut connection, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/questions/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Questions and their dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Questions not found")
    )
)]
pub async fn question_multiple_delete(
    State(state): State<AppState>,

    Json(question_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_questions(&mut connection, question_info).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/questions/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateQuestion],
    responses(
        (status = 200, description = "Quetions and their dependences updated successfully", body=[QuestionWithAnswers]),
        (status = 401, description = "Unauthorized to update Quetions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Quetions and their dependences not found")
    )
)]
pub async fn question_multiple_update(
    State(state): State<AppState>,

    Json(question_info): Json<Vec<UpdateQuestion>>,
) -> HandlerResult<Vec<QuestionWithAnswers>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_questions(&mut connection, question_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn question_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(question_create).get(question_list))
        .route("/multiple_delete", delete(question_multiple_delete))
        .route("/multiple_patch", patch(question_multiple_update))
}
