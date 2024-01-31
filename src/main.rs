#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_contrib;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenvy::dotenv;
use rocket::serde::json::{json, Value};
use std::env;

mod models;
mod routes;
mod schema;
mod services;

use routes::{
    answer, attribute, history, question, question_rule_group, system, user as user_routes,
};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug)]
struct AppState {
    db_pool: Pool,
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
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
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
            routes![user_routes::index, user_routes::create, user_routes::user],
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
        .register(
            "/",
            catchers![not_found, server_error, unprocessable_entity],
        )
        .manage(AppState { db_pool: pool })
}
