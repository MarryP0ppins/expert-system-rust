use crate::{
    models::question::{NewQuestionWithAnswersBody, Question, QuestionWithAnswers, UpdateQuestion},
    services::question::{
        create_question, get_questions, multiple_delete_questions, multiple_update_questions,
    },
    AppState,
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[post("/", format = "json", data = "<question_info>")]
pub fn question_create(
    state: &State<AppState>,
    question_info: Json<NewQuestionWithAnswersBody>,
) -> Result<Json<QuestionWithAnswers>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = create_question(&mut connection, question_info.0);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub fn question_list(
    state: &State<AppState>,
    system: i32,
) -> Result<Json<Vec<QuestionWithAnswers>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_questions(&mut connection, system);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_delete", format = "json", data = "<question_info>")]
pub fn questions_multiple_delete(
    state: &State<AppState>,
    question_info: Json<Vec<i32>>,
) -> Result<Value, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = multiple_delete_questions(&mut connection, question_info.0);

    match result {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_patch", format = "json", data = "<question_info>")]
pub fn question_multiple_update(
    state: &State<AppState>,
    question_info: Json<Vec<UpdateQuestion>>,
) -> Result<Json<Vec<Question>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = multiple_update_questions(&mut connection, question_info.0);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
