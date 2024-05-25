use crate::entity::{
    answers::{Entity as AnswerEntity, Model as AnswerModel},
    questions::{
        ActiveModel as QuestionActiveModel, Column as QuestionColumn, Entity as QuestionEntity,
        NewQuestionWithAnswersModel, QuestionWithAnswersModel, UpdateQuestionModel,
    },
};

use futures::future::try_join_all;
use sea_orm::*;

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

    let result = questions_with_answers
        .into_iter()
        .map(|(question, answers)| QuestionWithAnswersModel {
            id: question.id,
            system_id: question.system_id,
            body: question.body,
            answers,
            with_chooses: question.with_chooses,
        })
        .collect();

    Ok(result)
}

pub async fn create_questions<C>(
    db: &C,
    question_info: Vec<NewQuestionWithAnswersModel>,
) -> Result<Vec<QuestionWithAnswersModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_questions = db
        .transaction::<_, Vec<QuestionWithAnswersModel>, DbErr>(|txn| {
            Box::pin(async move {
                let new_question = question_info.into_iter().map(|question_raw| async move {
                    let new_question = QuestionActiveModel {
                        system_id: Set(question_raw.system_id),
                        body: Set(question_raw.body),
                        with_chooses: Set(question_raw.with_chooses),
                        ..Default::default()
                    };
                    let created_question = new_question.insert(txn).await;
                    match created_question {
                        Ok(result) => {
                            let values_to_create = question_raw
                                .answers_body
                                .into_iter()
                                .map(|answer_name| AnswerModel {
                                    id: -1,
                                    question_id: result.id,
                                    body: answer_name,
                                })
                                .collect();
                            let answers = create_answer(txn, values_to_create).await?;
                            Ok(QuestionWithAnswersModel {
                                id: result.id,
                                system_id: result.system_id,
                                body: result.body,
                                with_chooses: result.with_chooses,
                                answers,
                            })
                        }
                        Err(err) => Err(err),
                    }
                });

                let mut result = try_join_all(new_question).await?;
                result.sort_by(|a, b| a.id.cmp(&b.id));

                Ok(result)
            })
        })
        .await
        .or_else(|err| {
            Err(DbErr::Custom(format!(
                "Transaction error: {}",
                err.to_string()
            )))
        })?;

    Ok(new_questions)
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
        .map(|questions_for_update| async move {
            questions_for_update.into_active_model().update(db).await
        });

    let mut questions = try_join_all(updated_questions).await?;
    questions.sort_by(|a, b| a.id.cmp(&b.id));

    let questions_answers = questions.load_many(AnswerEntity, db).await?;

    let result = questions
        .into_iter()
        .zip(questions_answers)
        .map(|(question, question_answers)| {
            let mut answers = question_answers;
            answers.sort_by(|a, b| a.id.cmp(&b.id));
            QuestionWithAnswersModel {
                id: question.id,
                system_id: question.system_id,
                body: question.body,
                with_chooses: question.with_chooses,
                answers,
            }
        })
        .collect();

    Ok(result)
}
