use crate::{
    models::attribute_rule_group::{
        AttributeRuleGroupWithRulesAndAttributesValues,
        NewAttributeRuleGroupWithRulesAndAttributesValues,
    },
    services::attribute_rule_group::{
        create_attribute_rule_groups, get_attribute_rule_groups,
        multiple_delete_attribute_rule_groups,
    },
    utils::auth::cookie_check,
    AppState,
};
use diesel::{
    prelude::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[post("/", format = "json", data = "<attribute_rule_group_info>")]
pub fn attribute_rule_group_create(
    state: &State<AppState>,
    attribute_rule_group_info: Json<Vec<NewAttributeRuleGroupWithRulesAndAttributesValues>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeRuleGroupWithRulesAndAttributesValues>>, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match create_attribute_rule_groups(&mut connection, attribute_rule_group_info.0) {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub fn attribute_rule_group_list(
    state: &State<AppState>,
    system: i32,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<AttributeRuleGroupWithRulesAndAttributesValues>>, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match get_attribute_rule_groups(&mut connection, system) {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[post(
    "/multiple_delete",
    format = "json",
    data = "<attribute_rule_group_info>"
)]
pub fn attribute_rule_group_multiple_delete(
    state: &State<AppState>,
    attribute_rule_group_info: Json<Vec<i32>>,
    cookie: &CookieJar<'_>,
) -> Result<Value, Custom<Value>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>>;
    match state.db_pool.get() {
        Ok(ok) => connection = ok,
        Err(err) => {
            return Err(Custom(
                Status::InternalServerError,
                json!({"error":err.to_string(), "message":"Failed to get a database connection"})
                    .into(),
            ))
        }
    };

    match cookie_check(&mut connection, cookie) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match multiple_delete_attribute_rule_groups(&mut connection, attribute_rule_group_info.0) {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
