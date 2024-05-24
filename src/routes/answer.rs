use crate::{
    entity::answers::{Model as AnswerModel, UpdateAnswerModel},
    models::error::CustomErrors,
    pagination::AnswerListPagination,
    services::answer::{
        create_answer, get_answers, multiple_delete_answers, multiple_update_answers,
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
    path = "/answers",
    context_path ="/api/v1",
    request_body = [AnswerModel],
    responses(
        (status = 200, description = "Answers create successfully", body = [AnswerModel]),
        (status = 401, description = "Unauthorized to create Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn answer_create(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<AnswerModel>>,
) -> impl IntoResponse {
    match create_answer(&state.db_sea, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/answers",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Answers by query", body = [AnswerModel]),
        (status = 401, description = "Unauthorized to list Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AnswerListPagination
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn answer_list(
    State(state): State<AppState>,
    Query(pagination): Query<AnswerListPagination>,
) -> impl IntoResponse {
    let pagination: AnswerListPagination = pagination;

    match get_answers(&state.db_sea, pagination.question_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/answers/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Answers deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn answer_multiple_delete(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_answers(&state.db_sea, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/answers/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAnswer],
    responses(
        (status = 200, description = "Answers updated successfully", body = [AnswerModel]),
        (status = 401, description = "Unauthorized to update Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn answer_multiple_update(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<UpdateAnswerModel>>,
) -> impl IntoResponse {
    match multiple_update_answers(&state.db_sea, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn answer_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(answer_create).get(answer_list))
        .route("/multiple_delete", delete(answer_multiple_delete))
        .route("/multiple_patch", patch(answer_multiple_update))
}
