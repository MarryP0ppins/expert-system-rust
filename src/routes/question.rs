use crate::{
    models::{
        error::CustomErrors,
        question::{NewQuestionWithAnswersBody, UpdateQuestion},
        response_body::{ResponseBodyEmpty, ResponseBodyQuestions},
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
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/questions",
    context_path ="/api/v1",
    request_body = [NewQuestionWithAnswersBody],
    responses(
        (status = 200, description = "Questions and their dependences create successfully", body = ResponseBodyQuestions),
        (status = 401, description = "Unauthorized to create Questions and their dependences", body = ResponseBodyQuestions, example = json!(ResponseBodyQuestions::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn question_create(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<NewQuestionWithAnswersBody>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyQuestions::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_questions(&mut connection, question_info).await {
        Ok(result) => ResponseBodyQuestions::from(result),
        Err(err) => ResponseBodyQuestions::from(CustomErrors::DieselError {
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
        (status = 200, description = "List matching Questions and their dependences by query", body = ResponseBodyQuestions),
        (status = 401, description = "Unauthorized to list Questions and their dependences", body = ResponseBodyQuestions, example = json!(ResponseBodyQuestions::unauthorized_example()))
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
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyQuestions::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as QuestionListPagination;

    match get_questions(&mut connection, pagination.system_id).await {
        Ok(result) => ResponseBodyQuestions::from(result),
        Err(err) => ResponseBodyQuestions::from(CustomErrors::DieselError {
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
        (status = 200, description = "Questions and their dependences deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete Questions and their dependences", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "Questions not found")
    )
)]
#[debug_handler]
pub async fn question_multiple_delete(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_questions(&mut connection, question_info).await {
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
    path = "/questions/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateQuestion],
    responses(
        (status = 200, description = "Quetions and their dependences updated successfully", body = ResponseBodyQuestions),
        (status = 401, description = "Unauthorized to update Quetions and their dependences", body = ResponseBodyQuestions, example = json!(ResponseBodyQuestions::unauthorized_example())),
        (status = 404, description = "Quetions and their dependences not found")
    )
)]
#[debug_handler]
pub async fn question_multiple_update(
    State(state): State<AppState>,
    Json(question_info): Json<Vec<UpdateQuestion>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyQuestions::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_questions(&mut connection, question_info).await {
        Ok(result) => ResponseBodyQuestions::from(result),
        Err(err) => ResponseBodyQuestions::from(CustomErrors::DieselError {
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
