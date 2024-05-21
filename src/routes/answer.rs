use crate::{
    models::{
        answer::{NewAnswer, UpdateAnswer},
        error::CustomErrors,
    },
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
    request_body = [NewAnswer],
    responses(
        (status = 200, description = "Answers create successfully", body = [Answer]),
        (status = 401, description = "Unauthorized to create Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn answer_create(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<NewAnswer>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match create_answer(&mut connection, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Answers by query", body = [Answer]),
        (status = 401, description = "Unauthorized to list Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AnswerListPagination
    )
)]
#[debug_handler]
pub async fn answer_list(
    State(state): State<AppState>,
    Query(pagination): Query<AnswerListPagination>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    let pagination: AnswerListPagination = pagination;

    match get_answers(&mut connection, pagination.question_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "Answers deleted successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to delete Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    )
)]
#[debug_handler]
pub async fn answer_multiple_delete(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_delete_answers(&mut connection, answer_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "Answers updated successfully", body = [Answer]),
        (status = 401, description = "Unauthorized to update Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    )
)]
#[debug_handler]
pub async fn answer_multiple_update(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<UpdateAnswer>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_update_answers(&mut connection, answer_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
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
