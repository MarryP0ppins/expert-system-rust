use crate::{
    models::{
        answer::{Answer, NewAnswer, UpdateAnswer},
        error::CustomErrors,
        response_body::{ResponseBodyAnswer, ResponseBodyEmpty},
    },
    pagination::AnswerListPagination,
    services::answer::{
        create_answer, get_answers, multiple_delete_answers, multiple_update_answers,
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
    path = "/answers",
    context_path ="/api/v1",
    request_body = [NewAnswer],
    responses(
        (status = 200, description = "Answers create successfully", body = ResponseBodyAnswer),
        (status = 401, description = "Unauthorized to create Answers", body = ResponseBodyAnswer, example = json!(ResponseBodyAnswer::unauthorized_example()))
    )
)]
pub async fn answer_create(
    State(state): State<AppState>,

    Json(answer_info): Json<Vec<NewAnswer>>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

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
        (status = 200, description = "List matching Answers by query", body=ResponseBodyAnswer),
        (status = 401, description = "Unauthorized to list Answers", body = ResponseBodyAnswer, example = json!(ResponseBodyAnswer::unauthorized_example()))
    ),
    params(
        AnswerListPagination
    )
)]
pub async fn answer_list(
    State(state): State<AppState>,
    Query(pagination): Query<AnswerListPagination>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

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
        (status = 200, description = "Answers deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete Answers", body = ResponseBodyAnswer, example = json!(ResponseBodyAnswer::unauthorized_example())),
        (status = 404, description = "Answers not found")
    )
)]
pub async fn answer_multiple_delete(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_answers(&mut connection, answer_info).await {
        Ok(_) => Ok(Json(json!({"delete":"successful"}))),
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
        (status = 200, description = "Answers updated successfully", body=ResponseBodyAnswer),
        (status = 401, description = "Unauthorized to update Answers", body = ResponseBodyAnswer, example = json!(ResponseBodyAnswer::unauthorized_example())),
        (status = 404, description = "Answers not found")
    )
)]
pub async fn answer_multiple_update(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<UpdateAnswer>>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

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
