use crate::{
    models::history::{History, HistoryWithSystemAndUser, NewHistory},
    {services, AppState},
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;
use services::history::{create_history, delete_history, get_histories};

#[post("/", format = "json", data = "<history_info>")]
pub fn history_create(
    state: &State<AppState>,
    history_info: Json<NewHistory>,
) -> Result<Json<History>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = create_history(&mut connection, history_info);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>&<user>")]
pub fn history_list(
    state: &State<AppState>,
    system: Option<i32>,
    user: Option<i32>,
) -> Result<Json<Vec<HistoryWithSystemAndUser>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_histories(&mut connection, system, user);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[delete("/<history_id>")]
pub fn history_delete(state: &State<AppState>, history_id: i32) -> Result<Value, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = delete_history(&mut connection, history_id);

    match result {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
