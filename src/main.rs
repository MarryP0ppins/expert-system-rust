#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_contrib;

use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenvy::dotenv;
use rocket::serde::json::{json, Value};
use std::env;

mod models;
mod routes;
mod schema;
mod services;
mod utils;

use routes::{
    answer, attribute, attribute_rule_group, attribute_value, history, object, question,
    question_rule_group, system, user,
};

type AsyncPool = bb8::Pool<AsyncPgConnection>;

#[derive(Debug)]
struct AppState {
    //db_pool: Pool,
    db_pool: AsyncPool,
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({
        "status": "Unprocessable Entity",
        "reason": "Invalid JSON payload"
    })
}

#[catch(500)]
fn server_error() -> Value {
    json!({
        "status": "Server error",
        "reason": "Something went wrong. Please try again later"
    })
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create pool");

    rocket::build()
        .mount(
            "/system",
            routes![
                system::system_create,
                system::system_list,
                system::system_retrieve,
                system::system_partial_update,
                system::system_delete
            ],
        )
        .mount(
            "/user",
            routes![
                user::user_get,
                user::user_registration,
                user::user_logout,
                user::user_login,
            ],
        )
        .mount(
            "/history",
            routes![
                history::history_create,
                history::history_list,
                history::history_delete
            ],
        )
        .mount(
            "/question",
            routes![
                question::question_create,
                question::question_list,
                question::question_multiple_delete,
                question::question_multiple_update
            ],
        )
        .mount(
            "/answer",
            routes![
                answer::answer_create,
                answer::answer_list,
                answer::answer_multiple_delete,
                answer::answer_multiple_update
            ],
        )
        .mount(
            "/question-rule-group",
            routes![
                question_rule_group::question_rule_group_create,
                question_rule_group::question_rule_group_list,
                question_rule_group::question_rule_group_multiple_delete
            ],
        )
        .mount(
            "/attribute",
            routes![
                attribute::attribute_create,
                attribute::attribute_list,
                attribute::attribute_multiple_delete,
                attribute::attribute_multiple_update
            ],
        )
        .mount(
            "/attribute-value",
            routes![
                attribute_value::attribute_value_create,
                attribute_value::attribute_value_list,
                attribute_value::attribute_value_multiple_delete,
                attribute_value::attribute_value_multiple_update
            ],
        )
        .mount(
            "/attribute-rule-group",
            routes![
                attribute_rule_group::attribute_rule_group_create,
                attribute_rule_group::attribute_rule_group_list,
                attribute_rule_group::attribute_rule_group_multiple_delete
            ],
        )
        .mount(
            "/object",
            routes![
                object::object_create,
                object::object_list,
                object::object_multiple_delete,
                object::object_multiple_update
            ],
        )
        .register(
            "/",
            catchers![not_found, server_error, unprocessable_entity],
        )
        .manage(AppState { db_pool: pool })
}
