use crate::{
    models::question_rule_group::{
        NewQuestionRuleGroupWithRulesAndAnswers, QuestionRuleGroupWithRulesAndAnswers,
    },
    services::question_rule_group::{
        create_question_rule_group, get_question_rule_groups, multiple_delete_question_rule_groups,
    },
    AppState,
};
use diesel::{
    prelude::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
    State,
};
use rocket_contrib::json;

#[post("/", format = "json", data = "<question_rule_group_info>")]
pub fn question_rule_group_create(
    state: &State<AppState>,
    question_rule_group_info: Json<Vec<NewQuestionRuleGroupWithRulesAndAnswers>>,
) -> Result<Json<Vec<QuestionRuleGroupWithRulesAndAnswers>>, Custom<Value>> {
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
    
    match question_rule_group_info
        .0
        .into_iter()
        .map(|raw| create_question_rule_group(&mut connection, raw))
        .collect()
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}

#[get("/?<system>")]
pub fn question_rule_group_list(
    state: &State<AppState>,
    system: i32,
) -> Result<Json<Vec<QuestionRuleGroupWithRulesAndAnswers>>, Custom<Value>> {
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

    match get_question_rule_groups(&mut connection, system) {
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
pub fn question_rule_group_multiple_delete(
    state: &State<AppState>,
    question_rule_group_info: Json<Vec<i32>>,
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

    match multiple_delete_question_rule_groups(&mut connection, question_rule_group_info.0) {
        Ok(_) => Ok(json!({"delete":"successful"}).into()),
        Err(err) => Err(Custom(
            Status::BadRequest,
            json!({"error":err.to_string()}).into(),
        )),
    }
}
