use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;

use crate::{models::error::CustomErrors, utils::auth::cookie_check, AppState, URI_WITHOUT_AUTH};

pub async fn auth(
    State(state): State<AppState>,
    cookie: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, CustomErrors> {
    let mut connection;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

    if !URI_WITHOUT_AUTH
        .into_iter()
        .any(|uri| uri.uri == req.uri() && uri.method == req.method())
    {
        match cookie_check(&mut connection, cookie, &state.cookie_key).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
    }

    Ok(next.run(req).await)
}
