use crate::{
    constants::COOKIE_NAME,
    models::{
        error::CustomErrors,
        response_body::{ResponseBodyEmpty, ResponseBodyUser},
        user::{NewUser, UserLogin},
    },
    services::user::{create_user, get_user, login_user},
    AppState,
};
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use tower_cookies::{Cookie, Cookies};

#[utoipa::path(
    post,
    path = "/users/login",
    context_path ="/api/v1",
    request_body = UserLogin,
    responses(
        (status = 200, description = "User login successfully", body = ResponseBodyUser),
        (status = 400, description = "Invalid credantials provided", body = ResponseBodyUser, example = json!(ResponseBodyUser::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn user_login(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(user_info): Json<UserLogin>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyUser::from(CustomErrors::PoolConnectionError(err)),
    };

    match login_user(&mut connection, user_info, cookie, &state.cookie_key).await {
        Ok(result) => ResponseBodyUser::from(result),
        Err(err) => ResponseBodyUser::from(err),
    }
}

#[utoipa::path(
    post,
    path = "/users/logout",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "User logout successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
    )
)]
#[debug_handler]
pub async fn user_logout(cookie: Cookies) -> impl IntoResponse {
    cookie.remove(Cookie::new(COOKIE_NAME, ""));

    ResponseBodyEmpty {
        succsess: true,
        data: None,
        error: None,
    }
}

#[utoipa::path(
    post,
    path = "/users/registration",
    context_path ="/api/v1",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registration successfully", body = ResponseBodyUser),
        (status = 400, description = "Invalid credantials provided", body = ResponseBodyUser, example = json!(ResponseBodyUser::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn user_registration(
    State(state): State<AppState>,
    Json(user_info): Json<NewUser>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyUser::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_user(&mut connection, user_info).await {
        Ok(result) => ResponseBodyUser::from(result),
        Err(err) => ResponseBodyUser::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/users",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching User", body = ResponseBodyUser),
        (status = 401, description = "Unauthorized to User", body = ResponseBodyUser, example = json!(ResponseBodyUser::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn user_get(State(state): State<AppState>, cookie: Cookies) -> impl IntoResponse {
    let user_id: i32;
    match cookie
        .private(&state.cookie_key)
        .get(COOKIE_NAME)
        .map(|res| res.value().to_owned())
    {
        Some(res) => user_id = res.parse::<i32>().expect("Server Error"),
        None => {
            return ResponseBodyUser::from(CustomErrors::StringError {
                status: StatusCode::UNAUTHORIZED,
                error: "Not authorized".to_string(),
            })
        }
    };

    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyUser::from(CustomErrors::PoolConnectionError(err)),
    };

    match get_user(&mut connection, user_id).await {
        Ok(result) => ResponseBodyUser::from(result),
        Err(err) => ResponseBodyUser::from(CustomErrors::DieselError {
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
