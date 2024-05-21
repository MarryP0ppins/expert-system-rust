use crate::{
    models::rule_question_answer::NewRuleQuestionAnswer, schema::rule_question_answer::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_rule_question_answers(
    connection: &mut AsyncPgConnection,
    rule_question_answer_info: Vec<NewRuleQuestionAnswer>,
) -> Result<usize, Error> {
    Ok(insert_into(rule_question_answer)
        .values::<Vec<NewRuleQuestionAnswer>>(rule_question_answer_info)
        .execute(connection)
        .await?)
}

pub async fn multiple_delete_rule_question_answers(
    connection: &mut AsyncPgConnection,
    rule_question_answers_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(
        delete(rule_question_answer.filter(id.eq_any(rule_question_answers_ids)))
            .execute(connection)
            .await?,
    )
}
