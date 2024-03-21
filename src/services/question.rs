use crate::{
    models::{
        answer::{Answer, NewAnswer},
        question::{
            NewQuestion, NewQuestionWithAnswersBody, Question, QuestionWithAnswers, UpdateQuestion,
        },
    },
    schema::{answers, questions::dsl::*},
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};

pub async fn get_questions(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<QuestionWithAnswers>, Error> {
    let _questions = questions
        .filter(system_id.eq(system))
        .load::<Question>(connection)
        .await?;

    let _answers: Vec<Answer>;
    match Answer::belonging_to(&_questions)
        .load::<Answer>(connection)
        .await
    {
        Ok(ok) => _answers = ok,
        Err(_) => _answers = vec![],
    };

    let result = _answers
        .grouped_by(&_questions)
        .into_iter()
        .zip(_questions)
        .map(|(answers, question)| QuestionWithAnswers {
            id: question.id,
            answers,
            system_id: question.system_id,
            body: question.body,
            with_chooses: question.with_chooses,
        })
        .collect::<Vec<QuestionWithAnswers>>();

    Ok(result)
}

pub async fn create_questions(
    connection: &mut AsyncPgConnection,
    question_info: Vec<NewQuestionWithAnswersBody>,
) -> Result<Vec<QuestionWithAnswers>, Error> {
    let (answers_bodies, questions_raws) =
        question_info
            .into_iter()
            .fold((vec![], vec![]), |mut acc, raw| {
                acc.0.push(raw.answers_body);
                acc.1.push(NewQuestion {
                    system_id: raw.system_id,
                    body: raw.body,
                    with_chooses: raw.with_chooses,
                });
                acc
            });

    let mut new_questions: Vec<Question> = vec![];
    let mut _answers: Vec<Vec<Answer>> = vec![];

    match connection
        .transaction(|connection| {
            async {
                new_questions = insert_into(questions)
                    .values::<Vec<NewQuestion>>(questions_raws)
                    .get_results::<Question>(connection)
                    .await?;

                _answers = insert_into(answers::table)
                    .values::<Vec<NewAnswer>>(
                        answers_bodies
                            .into_iter()
                            .zip(&new_questions)
                            .flat_map(|(answer_bodies, question)| {
                                answer_bodies.into_iter().map(|value| NewAnswer {
                                    question_id: question.id,
                                    body: value,
                                })
                            })
                            .collect(),
                    )
                    .get_results::<Answer>(connection)
                    .await?
                    .grouped_by(&new_questions);

                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    let result = new_questions
        .into_iter()
        .zip(_answers)
        .map(|(question, answers)| QuestionWithAnswers {
            id: question.id,
            system_id: question.system_id,
            body: question.body,
            with_chooses: question.with_chooses,
            answers,
        })
        .collect();

    Ok(result)
}

pub async fn multiple_delete_questions(
    connection: &mut AsyncPgConnection,
    questions_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(questions.filter(id.eq_any(questions_ids)))
        .execute(connection)
        .await?)
}

pub async fn multiple_update_questions(
    connection: &mut AsyncPgConnection,
    questions_info: Vec<UpdateQuestion>,
) -> Result<Vec<QuestionWithAnswers>, Error> {
    let mut _questions: Vec<Question> = vec![];
    for question_raw in questions_info.into_iter() {
        match update(questions.find(question_raw.id))
            .set::<UpdateQuestion>(question_raw)
            .get_result::<Question>(connection)
            .await
        {
            Ok(result) => _questions.push(result),
            Err(err) => return Err(err),
        }
    }
    let _answers: Vec<Answer>;
    match Answer::belonging_to(&_questions)
        .load::<Answer>(connection)
        .await
    {
        Ok(ok) => _answers = ok,
        Err(_) => _answers = vec![],
    };

    let result = _answers
        .grouped_by(&_questions)
        .into_iter()
        .zip(_questions)
        .map(|(answers, question)| QuestionWithAnswers {
            id: question.id,
            answers,
            system_id: question.system_id,
            body: question.body,
            with_chooses: question.with_chooses,
        })
        .collect::<Vec<QuestionWithAnswers>>();

    Ok(result)
}
