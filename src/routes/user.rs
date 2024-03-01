use crate::{
    constants::COOKIE_NAME,
    models::{
        error::CustomErrors,
        user::{NewUser, UserLogin, UserWithoutPassword},
    },
    services::user::{create_user, get_user, login_user},
    AppState, HandlerResult,
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

#[utoipa::path(
    post,
    path = "/user/login",
    request_body = UserLogin,
    responses(
        (status = 200, description = "User login successfully", body=UserWithoutPassword),
        (status = 400, description = "Invalid credantials provided", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::BAD_REQUEST,
            error: "Invalid credantials provided".to_string(),
        }))
    )
)]
pub async fn user_login(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(user_info): Json<UserLogin>,
) -> HandlerResult<UserWithoutPassword> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match login_user(&mut connection, user_info, cookie, &state.cookie_key).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/user/logout",
    responses(
        (status = 200, description = "User logout successfully", body = Value, example = json!({"message":"You are logout"})),
    )
)]
pub async fn user_logout(cookie: Cookies) -> HandlerResult<Value> {
    cookie.remove(Cookie::new(COOKIE_NAME, ""));

    Ok(Json(json!({"message":"You are logout"})))
}

#[utoipa::path(
    post,
    path = "/user/registration",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registration successfully", body=UserWithoutPassword),
        (status = 400, description = "Invalid credantials provided", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::BAD_REQUEST,
            error: "Provided email or username already exist".to_string(),
        }))
    )
)]
pub async fn user_registration(
    State(state): State<AppState>,
    Json(user_info): Json<NewUser>,
) -> HandlerResult<UserWithoutPassword> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match create_user(&mut connection, user_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Matching User", body=UserWithoutPassword),
        (status = 401, description = "Unauthorized to User", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
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
            return Err(CustomErrors::StringError {
                status: StatusCode::UNAUTHORIZED,
                error: "Not authorized".to_string(),
            })
        }
    };

    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    match get_user(&mut connection, user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_get))
        .route("/logout", post(user_logout))
        .route("/login", post(user_login))
        .route("/registration", post(user_registration))
}
