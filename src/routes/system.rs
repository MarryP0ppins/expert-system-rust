use crate::{
    models::system::{NewSystem, System, UpdateSystem},
    services::system::{create_system, delete_system, get_system, get_systems, update_system},
    AppState,
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[post("/", format = "json", data = "<system_info>")]
pub fn system_create(
    state: &State<AppState>,
    system_info: Json<NewSystem>,
) -> Result<Json<System>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = create_system(&mut connection, system_info.0);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<name>")]
pub fn system_list(
    state: &State<AppState>,
    name: Option<String>,
) -> Result<Json<Vec<System>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_systems(&mut connection, name);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/<system_id>")]
pub fn system_retrieve(
    state: &State<AppState>,
    system_id: i32,
) -> Result<Json<System>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_system(&mut connection, system_id);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[patch("/<system_id>", format = "json", data = "<system_info>")]
pub fn system_partial_update(
    state: &State<AppState>,
    system_id: i32,
    system_info: Json<UpdateSystem>,
) -> Result<Json<System>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = update_system(&mut connection, system_id, system_info.0);

    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[delete("/<system_id>")]
pub fn system_delete(state: &State<AppState>, system_id: i32) -> Result<Value, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = delete_system(&mut connection, system_id);

    match result {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
