use entity::{
    answers::{Entity as AnswerEntity, Model as AnswerModel},
    questions::{
        ActiveModel as QuestionActiveModel, Column as QuestionColumn, Entity as QuestionEntity,
        NewQuestionWithAnswersModel, QuestionWithAnswersModel, UpdateQuestionModel,
    },
};

use futures::future::try_join_all;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    LoaderTrait, QueryFilter, Set, TransactionTrait,
};
use std::sync::Arc;

use super::answer::create_answer;

pub async fn get_questions<C>(
    db: &C,
    system_id: i32,
) -> Result<Vec<QuestionWithAnswersModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let questions_with_answers = QuestionEntity::find()
        .filter(QuestionColumn::SystemId.eq(system_id))
        .find_with_related(AnswerEntity)
        .all(db)
        .await?;

    let mut result = questions_with_answers
        .into_iter()
        .map(|(question, mut answers)| {
            answers.sort_by_key(|answer| answer.id);
            QuestionWithAnswersModel {
                id: question.id,
                system_id: question.system_id,
                body: question.body,
                answers,
                with_chooses: question.with_chooses,
            }
        })
        .collect::<Vec<QuestionWithAnswersModel>>();
    result.sort_by_key(|question| question.id);

    Ok(result)
}

pub async fn create_questions<C>(
    db: &C,
    question_info: Vec<NewQuestionWithAnswersModel>,
) -> Result<Vec<QuestionWithAnswersModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = db.begin().await?;
    let shared_txn = Arc::new(&txn);

    let new_questions = question_info.into_iter().map(|question_raw| {
        let txn_cloned = Arc::clone(&shared_txn);
        async move {
            let new_question = QuestionActiveModel {
                system_id: Set(question_raw.system_id),
                body: Set(question_raw.body),
                with_chooses: Set(question_raw.with_chooses),
                ..Default::default()
            };
            let created_question = new_question.insert(*txn_cloned).await?;

            let values_to_create = question_raw
                .answers_body
                .into_iter()
                .map(|answer_name| AnswerModel {
                    id: -1,
                    question_id: created_question.id,
                    body: answer_name,
                })
                .collect();
            let answers = create_answer(*txn_cloned, values_to_create).await?;
            Ok::<QuestionWithAnswersModel, DbErr>(QuestionWithAnswersModel {
                id: created_question.id,
                system_id: created_question.system_id,
                body: created_question.body,
                with_chooses: created_question.with_chooses,
                answers,
            })
        }
    });

    let mut result = try_join_all(new_questions).await?;
    result.sort_by_key(|question| question.id);

    txn.commit().await?;

    Ok(result)
}

pub async fn multiple_delete_questions<C>(db: &C, questions_ids: Vec<i32>) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(QuestionEntity::delete_many()
        .filter(QuestionColumn::Id.is_in(questions_ids))
        .exec(db)
        .await?
        .rows_affected)
}

pub async fn multiple_update_questions<C>(
    db: &C,
    questions_info: Vec<UpdateQuestionModel>,
) -> Result<Vec<QuestionWithAnswersModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let updated_questions = questions_info
        .into_iter()
        .map(|questions_for_update| questions_for_update.into_active_model().update(db));

    let mut questions = try_join_all(updated_questions).await?;
    questions.sort_by_key(|question| question.id);

    let questions_answers = questions.load_many(AnswerEntity, db).await?;

    let result = questions
        .into_iter()
        .zip(questions_answers)
        .map(|(question, mut question_answers)| {
            question_answers.sort_by_key(|answer| answer.id);
            QuestionWithAnswersModel {
                id: question.id,
                system_id: question.system_id,
                body: question.body,
                with_chooses: question.with_chooses,
                answers: question_answers,
            }
        })
        .collect();

    Ok(result)
}
