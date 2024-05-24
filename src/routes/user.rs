use crate::{
    constants::COOKIE_NAME,
    entity::users::{LoginUserModel, Model as UserModel, UpdateUserResponse},
    models::error::CustomErrors,
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
    request_body = LoginUserModel,
    responses(
        (status = 200, description = "User login successfully", body = UserModel),
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
    Json(user_info): Json<LoginUserModel>,
) -> impl IntoResponse {
    match login_user(&state.db_sea, user_info, cookie, &state.cookie_key).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
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
    request_body = UserModel,
    responses(
        (status = 200, description = "User registration successfully", body = UserModel),
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
    Json(user_info): Json<UserModel>,
) -> impl IntoResponse {
    match create_user(&state.db_sea, user_info, cookie, &state.cookie_key).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "Matching User", body = UserModel),
        (status = 401, description = "Unauthorized to User", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn user_get(State(state): State<AppState>, cookie: Cookies) -> impl IntoResponse {
    let user_id = cookie
        .private(&state.cookie_key)
        .get(COOKIE_NAME)
        .map(|res| res.value().to_owned().parse::<i32>())
        .ok_or(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Пользователь не авторизован".to_string(),
        })?
        .map_err(|err| CustomErrors::StringError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: err.to_string(),
        })?;

    match get_user(&state.db_sea, user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
        (status = 200, description = "Matching User", body = UserModel),
        (status = 401, description = "Unauthorized to User", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
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

    //ИСПРАВИТЬ ^^^^

    match update_user(&state.db_sea, user, user_cookie.id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
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
