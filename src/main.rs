#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

#[cfg(not(debug_assertions))]
use axum::routing::get;
use axum::{
    http::{HeaderValue, Method},
    middleware as axum_middleware, Router,
};
use constants::IMAGE_DIR;
use dotenv::dotenv;
use http::{
    header::{ACCEPT, CONTENT_TYPE, SET_COOKIE, X_CONTENT_TYPE_OPTIONS},
    HeaderName,
};
use middleware::{auth, handler_404};

use routes::{
    answer::answer_routes, attribute::attribute_routes, attribute_value::attribute_value_routes,
    clause::clause_routes, history::history_routes, object::object_routes,
    object_attribute_attributevalue::object_attribute_attributevalue_routes,
    question::question_routes, rule::rule_routes,
    rule_attribute_attributevalue::rule_attribute_attributevalue_routes,
    rule_question_answer::rule_question_answer_routes, system::system_routes, user::user_routes,
};
use sea_orm::{Database, DatabaseConnection};

use std::net::SocketAddr;
#[cfg(not(debug_assertions))]
use swagger::openapi;
#[cfg(debug_assertions)]
use swagger::ApiDoc;

use tower_cookies::{cookie::Key, CookieManagerLayer};
use tower_http::{cors::CorsLayer, services::ServeDir};
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

mod constants;
mod entity;
mod middleware;
mod models;
mod pagination;
mod routes;
mod schema;
mod services;
mod swagger;
mod utils;

#[derive(Clone)]
struct AppState {
    db_sea: DatabaseConnection,
    cookie_key: Key,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    }

    let database_url = dotenv!("DATABASE_URL");
    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .expect("Failed to create sea connection");

    let cookie_key = dotenv!("COOKIE_KEY");
    let secret_key = Key::from(cookie_key.as_bytes());

    let state = AppState {
        db_sea: db,
        cookie_key: secret_key,
    };

    let page_header = HeaderName::from_lowercase(b"x-pages").unwrap();
    let cors = CorsLayer::new()
        .allow_origin(dotenv!("ALLOW_ORIGIN").parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([CONTENT_TYPE, SET_COOKIE, ACCEPT, X_CONTENT_TYPE_OPTIONS])
        .expose_headers([page_header])
        .allow_credentials(true);

    let mut app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/user", user_routes())
                .nest("/systems", system_routes())
                .nest("/histories", history_routes())
                .nest("/questions", question_routes())
                .nest("/answers", answer_routes())
                .nest("/attributes", attribute_routes())
                .nest("/attributevalues", attribute_value_routes())
                .nest("/clauses", clause_routes())
                .nest("/rules", rule_routes())
                .nest("/objects", object_routes())
                .nest(
                    "/object-attribute-attributevalue",
                    object_attribute_attributevalue_routes(),
                )
                .nest(
                    "/rule-attribute-attributevalue",
                    rule_attribute_attributevalue_routes(),
                )
                .nest("/rule-question-answer", rule_question_answer_routes()),
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
