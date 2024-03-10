use crate::{
    models::{
        answer::{NewAnswer, UpdateAnswer},
        error::CustomErrors,
        response_body::{ResponseBodyAnswers, ResponseBodyEmpty},
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
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/answers",
    context_path ="/api/v1",
    request_body = [NewAnswer],
    responses(
        (status = 200, description = "Answers create successfully", body = ResponseBodyAnswers),
        (status = 401, description = "Unauthorized to create Answers", body = ResponseBodyAnswers, example = json!(ResponseBodyAnswers::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn answer_create(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<NewAnswer>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyAnswers::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_answer(&mut connection, answer_info).await {
        Ok(result) => ResponseBodyAnswers::from(result),
        Err(err) => ResponseBodyAnswers::from(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Answers by query", body = ResponseBodyAnswers),
        (status = 401, description = "Unauthorized to list Answers", body = ResponseBodyAnswers, example = json!(ResponseBodyAnswers::unauthorized_example()))
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
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyAnswers::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination: AnswerListPagination = pagination;

    match get_answers(&mut connection, pagination.question_id).await {
        Ok(result) => ResponseBodyAnswers::from(result),
        Err(err) => ResponseBodyAnswers::from(CustomErrors::DieselError {
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
        (status = 401, description = "Unauthorized to delete Answers", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "Answers not found")
    )
)]
#[debug_handler]
pub async fn answer_multiple_delete(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_answers(&mut connection, answer_info).await {
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

#[utoipa::path(
    patch,
    path = "/answers/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAnswer],
    responses(
        (status = 200, description = "Answers updated successfully", body = ResponseBodyAnswers),
        (status = 401, description = "Unauthorized to update Answers", body = ResponseBodyAnswers, example = json!(ResponseBodyAnswers::unauthorized_example())),
        (status = 404, description = "Answers not found")
    )
)]
#[debug_handler]
pub async fn answer_multiple_update(
    State(state): State<AppState>,
    Json(answer_info): Json<Vec<UpdateAnswer>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyAnswers::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_answers(&mut connection, answer_info).await {
        Ok(result) => ResponseBodyAnswers::from(result),
        Err(err) => ResponseBodyAnswers::from(CustomErrors::DieselError {
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
