use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_cookies::Cookies;

use crate::{
    constants::URI_WITHOUT_AUTH, entity::error::CustomErrors, utils::auth::cookie_check, AppState,
};

pub async fn auth(
    State(state): State<AppState>,
    cookie: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    if !URI_WITHOUT_AUTH
        .into_iter()
        .any(|uri| uri.uri == req.uri().path() && uri.method == req.method())
    {
        match cookie_check(&state.db_sea, cookie, &state.cookie_key).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
    }

    Ok(next.run(req).await)
}

pub async fn handler_404() -> impl IntoResponse {
    CustomErrors::StringError {
        status: StatusCode::NOT_FOUND,
        error: "Resource was not found.".to_owned(),
    }
}
