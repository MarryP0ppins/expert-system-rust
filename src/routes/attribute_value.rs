use crate::{
    models::attribute_value::{AttributeValue, NewAttributeValue, UpdateAttributeValue},
    services::attribute_value::{
        create_attributes_values, get_attribute_values, multiple_delete_attributes_values,
        multiple_update_attributes_values,
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

#[post("/", format = "json", data = "<attribute_value_info>")]
pub async fn attribute_value_create(
    state: &State<AppState>,
    attribute_value_info: Json<Vec<NewAttributeValue>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeValue>>, Custom<Value>> {
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

    match create_attributes_values(&mut connection, attribute_value_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<attribute>")]
pub async fn attribute_value_list(
    state: &State<AppState>,
    attribute: i32,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeValue>>, Custom<Value>> {
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

    match get_attribute_values(&mut connection, attribute).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_delete", format = "json", data = "<attribute_value_info>")]
pub async fn attribute_value_multiple_delete(
    state: &State<AppState>,
    attribute_value_info: Json<Vec<i32>>,
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

    match multiple_delete_attributes_values(&mut connection, attribute_value_info.0).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post("/multiple_patch", format = "json", data = "<attribute_value_info>")]
pub async fn attribute_value_multiple_update(
    state: &State<AppState>,
    attribute_value_info: Json<Vec<UpdateAttributeValue>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeValue>>, Custom<Value>> {
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

    match multiple_update_attributes_values(&mut connection, attribute_value_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
