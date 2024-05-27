use crate::entity::rule_question_answer::{
    ActiveModel as RuleQuestionAnswerActiveModel, Column as RuleQuestionAnswerColumn,
    Entity as RuleQuestionAnswerEntity, Model as RuleQuestionAnswerModel,
};
use futures::future::try_join_all;
use sea_orm::*;

pub async fn create_rule_question_answers<C>(
    db: &C,
    rule_question_answer_info: Vec<RuleQuestionAnswerModel>,
) -> Result<Vec<RuleQuestionAnswerModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_rule_question_answers =
        rule_question_answer_info
            .into_iter()
            .map(|new_rule_question_answer| {
                let model = RuleQuestionAnswerActiveModel {
                    rule_id: Set(new_rule_question_answer.rule_id),
                    question_id: Set(new_rule_question_answer.question_id),
                    answer_id: Set(new_rule_question_answer.answer_id),
                    ..Default::default()
                };
                model.insert(db)
            });

    let mut result = try_join_all(new_rule_question_answers).await?;
    result.sort_by_key(|rule_question_answer| rule_question_answer.id);

    Ok(result)
}

pub async fn multiple_delete_rule_question_answers<C>(
    db: &C,
    rule_question_answers_ids: Vec<i32>,
) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(RuleQuestionAnswerEntity::delete_many()
        .filter(RuleQuestionAnswerColumn::Id.is_in(rule_question_answers_ids))
        .exec(db)
        .await?
        .rows_affected)
}
