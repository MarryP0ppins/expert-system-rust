use crate::{
    models::{
        error::CustomErrors,
        question::{NewQuestionWithAnswersBody, UpdateQuestion},
    },
    pagination::QuestionListPagination,
    services::question::{
        create_questions, get_questions, multiple_delete_questions, multiple_update_questions,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};

#[utoipa::path(
    post,
    path = "/questions",
    context_path ="/api/v1",
    request_body = [NewQuestionWithAnswersBody],
    responses(
        (status = 200, description = "Questions and their dependences create successfully", body = [QuestionWithAnswers]),
        (status = 401, description = "Unauthorized to create Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn question_create(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<NewQuestionWithAnswersBody>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

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
        (status = 200, description = "List matching Questions and their dependences by query", body = [QuestionWithAnswers]),
        (status = 401, description = "Unauthorized to list Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        QuestionListPagination
    )
)]
#[debug_handler]
pub async fn question_list(
    State(state): State<AppState>,
    Query(pagination): Query<QuestionListPagination>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

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
        (status = 200, description = "Questions and their dependences deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Questions not found")
    )
)]
#[debug_handler]
pub async fn question_multiple_delete(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_delete_questions(&mut connection, question_info).await {
        Ok(_) => Ok(()),
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
        (status = 200, description = "Quetions and their dependences updated successfully", body = [QuestionWithAnswers]),
        (status = 401, description = "Unauthorized to update Quetions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Quetions and their dependences not found")
    )
)]
#[debug_handler]
pub async fn question_multiple_update(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<UpdateQuestion>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

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
