use crate::{
    constants::COOKIE_NAME,
    models::{
        error::CustomErrors,
        user::{NewUser, UpdateUserResponse, UserLogin},
    },
    services::user::{create_user, get_user, login_user, update_user},
    utils::auth::password_check,
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
use tower_cookies::{Cookie, Cookies};

#[utoipa::path(
    post,
    path = "/user/login",
    context_path ="/api/v1",
    request_body = UserLogin,
    responses(
        (status = 200, description = "User login successfully", body = UserWithoutPassword),
        (status = 400, description = "Invalid credantials provided", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn user_login(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(user_info): Json<UserLogin>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match login_user(&mut connection, user_info, cookie, &state.cookie_key).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/user/logout",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "User logout successfully", body = CustomErrors, example = json!(())),
    )
)]
#[debug_handler]
pub async fn user_logout(cookie: Cookies) -> impl IntoResponse {
    cookie.remove(Cookie::new(COOKIE_NAME, ""));

    ()
}

#[utoipa::path(
    post,
    path = "/user/registration",
    context_path ="/api/v1",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registration successfully", body = UserWithoutPassword),
        (status = 400, description = "Invalid credantials provided", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn user_registration(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(user_info): Json<NewUser>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match create_user(&mut connection, user_info, cookie, &state.cookie_key).await {
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
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching User", body = UserWithoutPassword),
        (status = 401, description = "Unauthorized to User", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
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
            return Err(CustomErrors::StringError {
                status: StatusCode::UNAUTHORIZED,
                error: "Not authorized".to_string(),
            })
        }
    };

    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match get_user(&mut connection, user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/user",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "Matching User", body = UserWithoutPassword),
        (status = 401, description = "Unauthorized to User", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn user_patch(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(user): Json<UpdateUserResponse>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    let user_cookie =
        password_check(&mut connection, cookie, &state.cookie_key, &user.password).await?;

    match update_user(&mut connection, user, user_cookie.id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_get).patch(user_patch))
        .route("/logout", post(user_logout))
        .route("/login", post(user_login))
        .route("/registration", post(user_registration))
}
