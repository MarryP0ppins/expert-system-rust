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

pub fn get_questions(
    connection: &mut PgConnection,
    system: i32,
) -> Result<Vec<QuestionWithAnswers>, Error> {
    let _questions: Vec<Question>;
    match questions
        .filter(system_id.eq(system))
        .load::<Question>(connection)
    {
        Ok(ok) => _questions = ok,
        Err(err) => return Err(err),
    };

    let _answers: Vec<Answer>;
    match Answer::belonging_to(&_questions).load::<Answer>(connection) {
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

pub fn create_questions(
    connection: &mut PgConnection,
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

    let new_questions: Vec<Question>;
    match insert_into(questions)
        .values::<Vec<NewQuestion>>(questions_raws)
        .get_results::<Question>(connection)
    {
        Ok(ok) => new_questions = ok,
        Err(err) => return Err(err),
    };

    let _answers: Vec<Vec<Answer>>;
    match insert_into(answers::table)
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
    {
        Ok(ok) => _answers = ok.grouped_by(&new_questions),
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

pub fn multiple_delete_questions(
    connection: &mut PgConnection,
    questions_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(questions.filter(id.eq_any(questions_ids))).execute(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_update_questions(
    connection: &mut PgConnection,
    questions_info: Vec<UpdateQuestion>,
) -> Result<Vec<QuestionWithAnswers>, Error> {
    let _questions: Vec<Question>;
    match questions_info
        .iter()
        .map(|question_raw| {
            update(questions.find(question_raw.id))
                .set::<UpdateQuestion>(question_raw.clone())
                .get_result::<Question>(connection)
        })
        .collect()
    {
        Ok(result) => _questions = result,
        Err(err) => return Err(err),
    }

    let _answers: Vec<Answer>;
    match Answer::belonging_to(&_questions).load::<Answer>(connection) {
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
