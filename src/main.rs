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

mod routes;
mod services;
mod models;
mod schema;

use routes::{system as system_routes, user as user_routes};

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
            "/systems",
            routes![system_routes::index, system_routes::create],
        )
        .mount("/user", routes![user_routes::index, user_routes::create]) //, get_users, create_role, create_user, update_user])
        .register("/", catchers![not_found, server_error])
        .manage(AppState { db_pool: pool })
}
