use crate::{
    models::user::{NewUser, User},
    {services, AppState},
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;
use services::user::{create_user, get_user, get_users};

#[get("/")]
pub fn index(state: &State<AppState>) -> Result<Json<Vec<User>>, Custom<Value>> {
    let mut connection = state
        .db_pool
        .get()
        .expect("Failed to get a database connection");
    let result = get_users(&mut connection);

    match result {
        Ok(user) => Ok(Json(user)),
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
        Ok(user) => Ok(Json(user)),
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
    let result = create_user(&mut connection, user_info);

    match result {
        Ok(user) => Ok(Json(user)),
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
