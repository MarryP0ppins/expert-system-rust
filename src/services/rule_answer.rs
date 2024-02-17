use crate::{
    models::rule_answer::{NewRuleAnswer, RuleAnswer},
    schema::rule_answer::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_rule_answers(
    connection: &mut AsyncPgConnection,
    rule_answer_info: Vec<NewRuleAnswer>,
) -> Result<(), Error> {
    match insert_into(rule_answer)
        .values::<Vec<NewRuleAnswer>>(rule_answer_info)
        .get_result::<RuleAnswer>(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

pub async fn multiple_delete_rule_answers(
    connection: &mut AsyncPgConnection,
    rule_answers_ids: Vec<i32>,
) -> Result<(), Error> {
    match delete(rule_answer.filter(id.eq_any(rule_answers_ids)))
        .execute(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
