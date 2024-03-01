use crate::{
    models::{
        answer::{Answer, NewAnswer, UpdateAnswer},
        error::CustomErrors,
    },
    pagination::AnswerListPagination,
    services::answer::{
        create_answer, get_answers, multiple_delete_answers, multiple_update_answers,
    },
    utils::auth::cookie_check,
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
use tower_cookies::Cookies;

#[utoipa::path(
    post,
    path = "/answer",
    request_body = [NewAnswer],
    responses(
        (status = 200, description = "Answers create successfully", body=[Answer]),
        (status = 401, description = "Unauthorized to create Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
pub async fn answer_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(answer_info): Json<Vec<NewAnswer>>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
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
    path = "/answer",
    responses(
        (status = 200, description = "List matching Answers by query", body=[Answer]),
        (status = 401, description = "Unauthorized to list Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    params(
        AnswerListPagination
    )
)]
pub async fn answer_list(
    State(state): State<AppState>,
    Query(pagination): Query<AnswerListPagination>,
    cookie: Cookies,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
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
    path = "/answer/multiple_delete",
    request_body = [i32],
    responses(
        (status = 200, description = "Answers deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    )
)]
pub async fn answer_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(answer_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
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
    path = "/answer/multiple_update",
    request_body = [UpdateAnswer],
    responses(
        (status = 200, description = "Answers updated successfully", body=[Answer]),
        (status = 401, description = "Unauthorized to update Answers", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "Answers not found")
    )
)]
pub async fn answer_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(answer_info): Json<Vec<UpdateAnswer>>,
) -> HandlerResult<Vec<Answer>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err),
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
