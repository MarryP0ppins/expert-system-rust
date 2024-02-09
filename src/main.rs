#[macro_use]
extern crate axum_macros;
extern crate diesel;

use axum::{http::StatusCode, response::Json, Router};
use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use routes::{history::history_routes, system::system_routes, user::user_routes};
use serde_json::Value;
use std::{env, net::SocketAddr};
use tower_cookies::{cookie::Key, CookieManagerLayer};

mod models;
mod pagination;
mod routes;
mod schema;
mod services;
mod utils;

pub const COOKIE_NAME: &str = "session_id";

type HandlerResult<T> = Result<Json<T>, (StatusCode, Json<Value>)>;
type AsyncPool = bb8::Pool<AsyncPgConnection>;

#[derive(Debug, Clone)]
struct AppState {
    db_pool: AsyncPool,
    cookie_key: Key,
}

/*
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
 */

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create pool");

    let mut secret_key = [0u8; 64];
    StdRng::from_entropy().fill(&mut secret_key);
    let secret_key = Key::from(&secret_key);

    let app = Router::new()
        .nest("/user", user_routes())
        .nest("/system", system_routes())
        .nest("/history", history_routes())
        .with_state(AppState {
            db_pool: pool,
            cookie_key: secret_key,
        })
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    /*
    rocket::build()
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
        .manage(AppState { db_pool: pool })*/
}
