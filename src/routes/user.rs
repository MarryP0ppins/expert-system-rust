use crate::{
    models::{
        error::CustomErrors,
        user::{NewUser, UserLogin, UserWithoutPassword},
    },
    services::user::{create_user, get_user, login_user},
    AppState, HandlerResult, COOKIE_NAME,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[debug_handler]
pub async fn user_login(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(user_info): Json<UserLogin>,
) -> HandlerResult<UserWithoutPassword> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match login_user(&mut connection, user_info, cookie, &state.cookie_key).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err.into()),
    }
}

#[debug_handler]
pub async fn user_logout(cookie: Cookies) -> HandlerResult<Value> {
    cookie.remove(Cookie::new(COOKIE_NAME, ""));

    Ok(json!({"message":"You are logout"}).into())
}

#[debug_handler]
pub async fn user_registration(
    State(state): State<AppState>,
    Json(user_info): Json<NewUser>,
) -> HandlerResult<UserWithoutPassword> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match create_user(&mut connection, user_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[debug_handler]
pub async fn user_get(
    State(state): State<AppState>,
    cookie: Cookies,
) -> HandlerResult<UserWithoutPassword> {
    let user_id: i32;
    match cookie
        .private(&state.cookie_key)
        .get(COOKIE_NAME)
        .map(|res| res.value().to_owned())
    {
        Some(res) => user_id = res.parse::<i32>().expect("Server Error"),
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                json!({"error":"Not authorized"}).into(),
            ))
        }
    };

    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match get_user(&mut connection, user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_get))
        .route("/logout", post(user_logout))
        .route("/login", post(user_login))
        .route("/registration", post(user_registration))
}
