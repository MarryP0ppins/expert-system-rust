use crate::{
    models::answer::{Answer, NewAnswer, UpdateAnswer},
    {services, AppState},
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;
use services::answer::{
    create_answer, get_answers, multiple_delete_answers, multiple_update_answers,
};

#[post("/", format = "json", data = "<answer_info>")]
pub fn answer_create(
    state: &State<AppState>,
    answer_info: Json<NewAnswer>,
) -> Result<Json<Answer>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = create_answer(&mut connection, answer_info);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<question>")]
pub fn answer_list(
    state: &State<AppState>,
    question: i32,
) -> Result<Json<Vec<Answer>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_answers(&mut connection, question);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_delete", format = "json", data = "<answer_info>")]
pub fn answer_multiple_delete(
    state: &State<AppState>,
    answer_info: Json<Vec<i32>>,
) -> Result<Value, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = multiple_delete_answers(&mut connection, answer_info.0);

    match result {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_patch", format = "json", data = "<answer_info>")]
pub fn answer_multiple_update(
    state: &State<AppState>,
    answer_info: Json<Vec<UpdateAnswer>>,
) -> Result<Json<Vec<Answer>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = multiple_update_answers(&mut connection, answer_info.0);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
