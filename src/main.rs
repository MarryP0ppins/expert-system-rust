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

use routes::{answer, history, question, system, user as user_routes};

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
                question::questions_multiple_delete,
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
        .register("/", catchers![not_found, server_error])
        .manage(AppState { db_pool: pool })
}
