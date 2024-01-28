use crate::{
    models::user::{NewUser, User},
    services::user::{create_user, get_user, get_users},
    AppState,
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[get("/")]
pub fn index(state: &State<AppState>) -> Result<Json<Vec<User>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_users(&mut connection);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/<id>")]
pub fn user(state: &State<AppState>, id: i32) -> Result<Json<User>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_user(&mut connection, id);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/", format = "json", data = "<user_info>")]
pub fn create(
    state: &State<AppState>,
    user_info: Json<NewUser>,
) -> Result<Json<User>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = create_user(&mut connection, user_info.0);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
