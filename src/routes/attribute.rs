use crate::{
    models::attribute::{
        AttributeWithAttributeValues, NewAttributeWithAttributeValuesName, UpdateAttribute,
    },
    services::attribute::{
        create_attributes, get_attributes, multiple_delete_attributes, multiple_update_attributes,
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

#[post("/", format = "json", data = "<attribute_info>")]
pub async fn attribute_create(
    state: &State<AppState>,
    attribute_info: Json<Vec<NewAttributeWithAttributeValuesName>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeWithAttributeValues>>, Custom<Value>> {
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

    match create_attributes(&mut connection, attribute_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub async fn attribute_list(
    state: &State<AppState>,
    system: i32,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeWithAttributeValues>>, Custom<Value>> {
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

    match get_attributes(&mut connection, system).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_delete", format = "json", data = "<attribute_info>")]
pub async fn attribute_multiple_delete(
    state: &State<AppState>,
    attribute_info: Json<Vec<i32>>,
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

    match multiple_delete_attributes(&mut connection, attribute_info.0).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_patch", format = "json", data = "<attribute_info>")]
pub async fn attribute_multiple_update(
    state: &State<AppState>,
    attribute_info: Json<Vec<UpdateAttribute>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeWithAttributeValues>>, Custom<Value>> {
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

    match multiple_update_attributes(&mut connection, attribute_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
