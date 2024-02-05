use crate::{
    models::user::{NewUser, UserLogin, UserWithoutPassword},
    services::user::{create_user, get_user, login_user},
    AppState,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};

use rocket_contrib::json;

#[post("/login", format = "json", data = "<user_info>")]
pub async fn user_login(
    state: &State<AppState>,
    user_info: Json<UserLogin>,
    cookie: &CookieJar<'_>,
) -> Result<Json<UserWithoutPassword>, Custom<Value>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match login_user(&mut connection, user_info.0, &cookie).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err),
    }
}

#[post("/logout")]
pub async fn user_logout(cookie: &CookieJar<'_>) -> Result<Value, Custom<Value>> {
    cookie.remove_private("session_id");

    Ok(json!({"messege":"you are logout"}).into())
}

#[post("/registration", format = "json", data = "<user_info>")]
pub async fn user_registration(
    state: &State<AppState>,
    user_info: Json<NewUser>,
) -> Result<Json<UserWithoutPassword>, Custom<Value>> {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match create_user(&mut connection, user_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/")]
pub async fn user_get(
    state: &State<AppState>,
    cookie: &CookieJar<'_>,
) -> Result<Json<UserWithoutPassword>, Custom<Value>> {
    let user_id: i32;
    match cookie
        .get_private("session_id")
        .map(|res| res.value().to_owned())
    {
        Some(res) => user_id = res.parse::<i32>().expect("Server Error"),
        None => {
            return Err(Custom(
                Status::Unauthorized,
                json!({"error":"Not authorized"}).into(),
            ))
        }
    };

    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match get_user(&mut connection, user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
