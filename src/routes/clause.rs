use crate::{
    models::{
        clause::{Clause, NewClause, UpdateClause},
        error::CustomErrors,
    },
    services::clause::{
        create_clauses, get_clauses, multiple_delete_clauses, multiple_update_clauses,
    },
    utils::auth::cookie_check,
    AppState, HandlerResult,
};
use axum::{
    extract::{Query, State},
    routing::post,
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use serde_json::{json, Value};
use tower_cookies::Cookies;

#[debug_handler]
pub async fn clause_create(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(clause_info): Json<Vec<NewClause>>,
) -> HandlerResult<Vec<Clause>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match create_clauses(&mut connection, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
pub async fn clause_list(
    State(state): State<AppState>,
    Query(rule): Query<i32>,
    cookie: Cookies,
) -> HandlerResult<Vec<Clause>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match get_clauses(&mut connection, rule).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
pub async fn clause_multiple_delete(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(clause_info): Json<Vec<i32>>,
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

    match multiple_delete_clauses(&mut connection, clause_info).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

#[debug_handler]
pub async fn clause_multiple_update(
    State(state): State<AppState>,
    cookie: Cookies,
    Json(clause_info): Json<Vec<UpdateClause>>,
) -> HandlerResult<Vec<Clause>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err).into()),
    };

    match cookie_check(&mut connection, cookie, &state.cookie_key).await {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    };

    match multiple_update_clauses(&mut connection, clause_info).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::DieselError {
            error: err,
            message: None,
        }
        .into()),
    }
}

pub fn clause_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(clause_create).get(clause_list))
        .route("/multiple_delete", post(clause_multiple_delete))
        .route("/multiple_patch", post(clause_multiple_update))
}
