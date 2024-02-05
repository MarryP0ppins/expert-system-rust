use crate::{
    models::question::{NewQuestionWithAnswersBody, QuestionWithAnswers, UpdateQuestion},
    services::question::{
        create_questions, get_questions, multiple_delete_questions, multiple_update_questions,
    },
    utils::auth::cookie_check,
    AppState,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[post("/", format = "json", data = "<question_info>")]
pub async fn question_create(
    state: &State<AppState>,
    question_info: Json<Vec<NewQuestionWithAnswersBody>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<QuestionWithAnswers>>, Custom<Value>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match create_questions(&mut connection, question_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub async fn question_list(
    state: &State<AppState>,
    system: i32,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<QuestionWithAnswers>>, Custom<Value>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match get_questions(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_delete", format = "json", data = "<question_info>")]
pub async fn question_multiple_delete(
    state: &State<AppState>,
    question_info: Json<Vec<i32>>,
    cookie: &CookieJar<'_>,
) -> Result<Value, Custom<Value>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_delete_questions(&mut connection, question_info.0).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_patch", format = "json", data = "<question_info>")]
pub async fn question_multiple_update(
    state: &State<AppState>,
    question_info: Json<Vec<UpdateQuestion>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<QuestionWithAnswers>>, Custom<Value>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_update_questions(&mut connection, question_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
