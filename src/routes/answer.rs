use crate::{
    models::answer::{Answer, NewAnswer, UpdateAnswer},
    services::answer::{
        create_answer, get_answers, multiple_delete_answers, multiple_update_answers,
    },
    utils::auth::cookie_check,
    AppState,
};
use diesel::{
    prelude::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[post("/", format = "json", data = "<answer_info>")]
pub fn answer_create(
    state: &State<AppState>,
    answer_info: Json<Vec<NewAnswer>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<Answer>>, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match create_answer(&mut connection, answer_info.0) {
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
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<Answer>>, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match get_answers(&mut connection, question) {
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
    cookie: &CookieJar<'_>,
) -> Result<Value, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_delete_answers(&mut connection, answer_info.0) {
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
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<Answer>>, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_update_answers(&mut connection, answer_info.0) {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
