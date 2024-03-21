extern crate diesel;

#[cfg(not(debug_assertions))]
use axum::routing::get;
use axum::{
    http::{HeaderValue, Method},
    middleware as axum_middleware, Router,
};
use constants::IMAGE_DIR;
use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenvy::dotenv;
use middleware::{auth, handler_404};
use rand::{rngs::StdRng, Rng, SeedableRng};
use routes::{
    answer::answer_routes, attribute::attribute_routes, attribute_value::attribute_value_routes,
    clause::clause_routes, history::history_routes, object::object_routes,
    question::question_routes, rule::rule_routes, rule_answer::rule_answer_routes,
    rule_attributevalue::rule_attributevalue_routes, system::system_routes, user::user_routes,
};

use std::{env, net::SocketAddr};
#[cfg(not(debug_assertions))]
use swagger::openapi;
#[cfg(debug_assertions)]
use swagger::ApiDoc;

use tower_cookies::{cookie::Key, CookieManagerLayer};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

mod constants;
mod middleware;
mod models;
mod pagination;
mod routes;
mod schema;
mod services;
mod swagger;
mod utils;

type AsyncPool = bb8::Pool<AsyncPgConnection>;

#[derive(Clone)]
struct AppState {
    db_pool: AsyncPool,
    cookie_key: Key,
}

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

    let cors = CorsLayer::new()
        .allow_origin(
            env::var("ALLOW_ORIGIN")
                .expect("ALLOW_ORIGIN must be set")
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any); //.allow_credentials(true);

    let mut app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/users", user_routes())
                .nest("/systems", system_routes())
                .nest("/histories", history_routes())
                .nest("/questions", question_routes())
                .nest("/answers", answer_routes())
                .nest("/attributes", attribute_routes())
                .nest("/attributevalues", attribute_value_routes())
                .nest("/clauses", clause_routes())
                .nest("/rules", rule_routes())
                .nest("/objects", object_routes())
                .nest("/rule-attributevalues", rule_attributevalue_routes())
                .nest("/rule-answers", rule_answer_routes()),
        )
        .layer(axum_middleware::from_fn_with_state(state.clone(), auth))
        .nest_service("/api/v1/images", ServeDir::new(IMAGE_DIR))
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .fallback(handler_404)
        .layer(cors);

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
