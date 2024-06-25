use crate::{
    error::CustomErrors,
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
use entity::questions::{NewQuestionWithAnswersModel, UpdateQuestionModel};

#[utoipa::path(
    post,
    path = "/questions",
    context_path ="/api/v1",
    request_body = [NewQuestionWithAnswersModel],
    responses(
        (status = 200, description = "Questions and their dependences create successfully", body = [QuestionWithAnswersModel]),
        (status = 401, description = "Unauthorized to create Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn question_create(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<NewQuestionWithAnswersModel>>,
) -> impl IntoResponse {
    match create_questions(&state.db_sea, question_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "List matching Questions and their dependences by query", body = [QuestionWithAnswersModel]),
        (status = 401, description = "Unauthorized to list Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        QuestionListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn question_list(
    State(state): State<AppState>,
    Query(pagination): Query<QuestionListPagination>,
) -> impl IntoResponse {
    match get_questions(&state.db_sea, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "Questions and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Questions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Questions not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn question_multiple_delete(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_questions(&state.db_sea, question_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/questions/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateQuestionModel],
    responses(
        (status = 200, description = "Quetions and their dependences updated successfully", body = [QuestionWithAnswersModel]),
        (status = 401, description = "Unauthorized to update Quetions and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Quetions and their dependences not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn question_multiple_update(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<UpdateQuestionModel>>,
) -> impl IntoResponse {
    match multiple_update_questions(&state.db_sea, question_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
