use crate::{
    models::question_rule_group::{
        NewQuestionRuleGroupWithRulesAndAnswers, QuestionRuleGroupWithRulesAndAnswers,
    },
    services::question_rule_group::{
        create_question_rule_groups, get_question_rule_groups, multiple_delete_question_rule_groups,
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

#[post("/", format = "json", data = "<question_rule_group_info>")]
pub async fn question_rule_group_create(
    state: &State<AppState>,
    question_rule_group_info: Json<Vec<NewQuestionRuleGroupWithRulesAndAnswers>>,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<QuestionRuleGroupWithRulesAndAnswers>>, Custom<Value>> {
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

    match create_question_rule_groups(&mut connection, question_rule_group_info.0).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub async fn question_rule_group_list(
    state: &State<AppState>,
    system: i32,
    cookie: &CookieJar<'_>,
) -> Result<Json<Vec<QuestionRuleGroupWithRulesAndAnswers>>, Custom<Value>> {
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

    match get_question_rule_groups(&mut connection, system).await {
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
    data = "<question_rule_group_info>"
)]
pub async fn question_rule_group_multiple_delete(
    state: &State<AppState>,
    question_rule_group_info: Json<Vec<i32>>,
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

    match multiple_delete_question_rule_groups(&mut connection, question_rule_group_info.0).await {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
