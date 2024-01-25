use crate::{
    models::system::{NewSystem, System},
    {services, AppState},
};
//use expert_system_rust::models::system::{NewSystem, System};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;
use services::system::{create_system, get_systems};

#[get("/")]
pub fn index(state: &State<AppState>) -> Result<Json<Vec<System>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_systems(&mut connection);

    match result {
        Ok(system) => Ok(Json(system)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/", format = "json", data = "<system_info>")]
pub fn create(
    state: &State<AppState>,
    system_info: Json<NewSystem>,
) -> Result<Json<Vec<System>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = create_system(&mut connection, system_info);

    match result {
        Ok(system) => Ok(Json(system)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
/*
#[post("/users/add-role", format = "json", data = "<role_info>")]
pub fn create_role(role_info: Json<NewSystem>) -> Value {
    services::users::add_role(&role_info.role_name)
}

#[post("/users/create-user", format = "json", data = "<user_info>")]
pub fn create_user(user_info: Json<UserInputUser>) -> Value {
    services::users::create_user(&user_info)
}

#[put("/users/update", format = "json", data = "<user_info>")]
pub fn update_user(user_info: Json<UserInputUpdateUser>) -> Value {
    services::users::update_user(&user_info)
} */
