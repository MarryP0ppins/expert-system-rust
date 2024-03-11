use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_cookies::Cookies;

use crate::{
    constants::URI_WITHOUT_AUTH,
    models::{
        error::CustomErrors,
        response_body::{ResponseBodyEmpty, ResponseBodyError},
    },
    utils::auth::cookie_check,
    AppState,
};

pub async fn auth(
    State(state): State<AppState>,
    cookie: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let mut connection;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(
                err,
            )))
        }
    };

    if !URI_WITHOUT_AUTH
        .into_iter()
        .any(|uri| uri.uri == req.uri() && uri.method == req.method())
    {
        match cookie_check(&mut connection, cookie, &state.cookie_key).await {
            Ok(_) => (),
            Err(err) => return Err(ResponseBodyEmpty::from(err)),
        };
    }

    Ok(next.run(req).await)
}

pub async fn handler_404() -> impl IntoResponse {
    ResponseBodyEmpty {
        succsess: false,
        data: None,
        error: Some(ResponseBodyError {
            status: StatusCode::NOT_FOUND,
            error: "Resource was not found.".to_owned(),
            extra: None,
        }),
    }
}
