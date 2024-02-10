extern crate diesel;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use routes::{
    answer::answer_routes, attribute::attribute_routes,
    attribute_rule_group::attribute_rule_group_routes, attribute_value::attribute_value_routes,
    history::history_routes, object::object_routes, question::question_routes,
    question_rule_group::question_rule_group_routes, system::system_routes, user::user_routes,
};
use serde_json::{json, Value};
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

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": "error",
            "reason": "Resource was not found."
        })),
    )
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
        .nest("/question", question_routes())
        .nest("/answer", answer_routes())
        .nest("/question-rule-group", question_rule_group_routes())
        .nest("/attribute", attribute_routes())
        .nest("/attribute_value", attribute_value_routes())
        .nest("/attribute-rule-group", attribute_rule_group_routes())
        .nest("/object", object_routes())
        .with_state(AppState {
            db_pool: pool,
            cookie_key: secret_key,
        })
        .layer(CookieManagerLayer::new())
        .fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    /*
    .register(
        "/",
        catchers![not_found, server_error, unprocessable_entity],
    )*/
}
