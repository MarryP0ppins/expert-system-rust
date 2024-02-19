use crate::{
    models::{
        error::CustomErrors,
        rule::{NewRule, RuleWithClausesAndEffects},
    },
    pagination::RuleListPagination,
    services::rule::{create_rule, get_rules, multiple_delete_rules},
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[utoipa::path(
    post,
    path = "/rule",
    request_body = [NewRule],
    responses(
        (status = 200, description = "Rule create successfully", body=[RuleWithClausesAndEffects]),
        (status = 401, description = "Unauthorized to create Rule", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    )
)]
pub async fn rule_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(rule_info): Json<NewRule>,
) -> HandlerResult<RuleWithClausesAndEffects> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_rule(&mut connection, rule_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    get,
    path = "/rule",
    responses(
        (status = 200, description = "List matching Rules and their dependences by query", body=[RuleWithClausesAndEffects]),
        (status = 401, description = "Unauthorized to list Rules and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        }))
    ),
    params(
        RuleListPagination
    )
)]
pub async fn rule_list(
    State(state): State<AppState>,
    Query(pagination): Query<RuleListPagination>,
    cookie: Cookies,
) -> HandlerResult<Vec<RuleWithClausesAndEffects>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };
    let pagination = pagination as RuleListPagination;

    match get_rules(&mut connection, pagination.system_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[utoipa::path(
    delete,
    path = "/rule/multiple_delete",
    request_body = [i32],
    responses(
        (status = 200, description = "Rules and their dependences deleted successfully", body = Value, example = json!({"delete":"successful"})),
        (status = 401, description = "Unauthorized to delete Rules and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized",
        })),
        (status = 404, description = "Rules not found")
    )
)]
pub async fn rule_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(rule_info): Json<Vec<i32>>,
) -> HandlerResult<Value> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_delete_rules(&mut connection, rule_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub fn rule_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(rule_create).get(rule_list))
        .route("/multiple_delete", delete(rule_multiple_delete))
}
