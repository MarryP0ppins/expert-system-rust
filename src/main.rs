extern crate axum_macros;
extern crate diesel;

#[cfg(not(debug_assertions))]
use axum::routing::get;
use axum::{
    http::{Method, StatusCode},
    middleware as axum_middleware,
    response::{IntoResponse, Json},
    Router,
};
use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenvy::dotenv;
use middleware::auth;
use models::error::CustomErrors;
use rand::{rngs::StdRng, Rng, SeedableRng};
use routes::{
    answer::answer_routes, attribute::attribute_routes, attribute_value::attribute_value_routes,
    clause::clause_routes, history::history_routes, object::object_routes,
    question::question_routes, rule::rule_routes, rule_answer::rule_answer_routes,
    rule_attributevalue::rule_attributevalue_routes, system::system_routes, user::user_routes,
};
use serde_json::json;
use std::{env, net::SocketAddr};
#[cfg(not(debug_assertions))]
use swagger::openapi;
#[cfg(debug_assertions)]
use swagger::ApiDoc;
use tower_cookies::{cookie::Key, CookieManagerLayer};
use tower_http::services::ServeDir;
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

mod middleware;
mod models;
mod pagination;
mod routes;
mod schema;
mod services;
mod swagger;
mod utils;

pub const COOKIE_NAME: &str = "session_id";
pub const IMAGE_DIR: &str = "./images";
pub const URI_WITHOUT_AUTH: [UriInfo; 4] = [
    UriInfo {
        uri: "/user/login",
        method: Method::POST,
    },
    UriInfo {
        uri: "/system",
        method: Method::GET,
    },
    UriInfo {
        uri: "/user/logout",
        method: Method::POST,
    },
    UriInfo {
        uri: "/user/registration",
        method: Method::POST,
    },
];
type HandlerResult<T> = Result<Json<T>, CustomErrors>;
type AsyncPool = bb8::Pool<AsyncPgConnection>;

#[derive(Debug)]
pub struct UriInfo<'a> {
    uri: &'a str,
    method: Method,
}

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

    let state = AppState {
        db_pool: pool,
        cookie_key: secret_key,
    };

    let mut app = Router::new()
        .nest("/user", user_routes())
        .nest("/system", system_routes())
        .nest("/history", history_routes())
        .nest("/question", question_routes())
        .nest("/answer", answer_routes())
        .nest("/attribute", attribute_routes())
        .nest("/attributevalue", attribute_value_routes())
        .nest("/clause", clause_routes())
        .nest("/rule", rule_routes())
        .nest("/object", object_routes())
        .nest("/rule-attributevalue", rule_attributevalue_routes())
        .nest("/rule-answer", rule_answer_routes())
        .nest_service("/images", ServeDir::new(IMAGE_DIR))
        .layer(axum_middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .fallback(handler_404);

    #[cfg(not(debug_assertions))]
    {
        app = app.route("/api-docs/openapi.json", get(openapi))
    }

    #[cfg(debug_assertions)]
    {
        app = app
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
