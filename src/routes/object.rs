use crate::{
    models::object::{NewObjectWithAttributesValueIds, ObjectWithAttributesValues, UpdateObject},
    services::object::{
        create_objects, get_objects, multiple_delete_objects, multiple_update_objects,
    },
    utils::auth::cookie_check,
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

#[post("/", format = "json", data = "<object_info>")]
pub async fn object_create(
    state: &State<AppState>,
    object_info: Json<Vec<NewObjectWithAttributesValueIds>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<ObjectWithAttributesValues>>, Custom<Value>> {
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

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match create_objects(&mut connection, object_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub async fn object_list(
    state: &State<AppState>,
    system: i32,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<ObjectWithAttributesValues>>, Custom<Value>> {
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

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match get_objects(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_delete", format = "json", data = "<object_info>")]
pub async fn object_multiple_delete(
    state: &State<AppState>,
    object_info: Json<Vec<i32>>,
    cookie: &CookieJar<'_>,
) -> Result<Value, Custom<Value>> {
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

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_delete_objects(&mut connection, object_info.0).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_patch", format = "json", data = "<object_info>")]
pub async fn object_multiple_update(
    state: &State<AppState>,
    object_info: Json<Vec<UpdateObject>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<ObjectWithAttributesValues>>, Custom<Value>> {
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

    match cookie_check(&mut connection, cookie).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_update_objects(&mut connection, object_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
