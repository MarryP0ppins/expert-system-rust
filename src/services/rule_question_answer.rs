use crate::{
    models::rule_question_answer::{NewRuleQuestionAnswer, RuleQuestionAnswer},
    schema::rule_question_answer::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_rule_question_answers(
    connection: &mut AsyncPgConnection,
    rule_question_answer_info: Vec<NewRuleQuestionAnswer>,
) -> Result<(), Error> {
    match insert_into(rule_question_answer)
        .values::<Vec<NewRuleQuestionAnswer>>(rule_question_answer_info)
        .get_result::<RuleQuestionAnswer>(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

pub async fn multiple_delete_rule_question_answers(
    connection: &mut AsyncPgConnection,
    rule_question_answers_ids: Vec<i32>,
) -> Result<(), Error> {
    match delete(rule_question_answer.filter(id.eq_any(rule_question_answers_ids)))
        .execute(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
