use futures::future::try_join_all;
use sea_orm::*;

use crate::entity::answers::{
    ActiveModel as AnswerActiveModel, Column as AnswerColumn, Entity as AnswerEntity,
    Model as AnswerModel, UpdateAnswerModel,
};

pub async fn get_answers<C>(db: &C, question_id: i32) -> Result<Vec<AnswerModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(AnswerEntity::find()
        .filter(AnswerColumn::QuestionId.eq(question_id))
        .order_by_asc(AnswerColumn::Id)
        .all(db)
        .await?)
}

pub async fn create_answer<C>(
    db: &C,
    answer_info: Vec<AnswerModel>,
) -> Result<Vec<AnswerModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_answers = answer_info.into_iter().map(|new_answer| async move {
        let model = AnswerActiveModel {
            question_id: Set(new_answer.question_id),
            body: Set(new_answer.body),
            ..Default::default()
        };
        model.insert(db).await
    });

    let mut result = try_join_all(new_answers).await?;

    result.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(result)
}

pub async fn multiple_delete_answers<C>(db: &C, answers_ids: Vec<i32>) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(AnswerEntity::delete_many()
        .filter(AnswerColumn::Id.is_in(answers_ids))
        .exec(db)
        .await?
        .rows_affected)
}

pub async fn multiple_update_answers<C>(
    db: &C,
    answer_info: Vec<UpdateAnswerModel>,
) -> Result<Vec<AnswerModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_answers = answer_info.into_iter().map(|answer_for_update| async move {
        answer_for_update.into_active_model().update(db).await
    });

    let mut result = try_join_all(new_answers).await?;

    result.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(result)
}
