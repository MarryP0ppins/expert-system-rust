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

pub fn create_question(
    connection: &mut PgConnection,
    question_info: NewQuestionWithAnswersBody,
) -> Result<QuestionWithAnswers, Error> {
    let answers_body = question_info.answers_body;
    let new_question: Question;
    match insert_into(questions)
        .values::<NewQuestion>(NewQuestion {
            system_id: question_info.system_id,
            body: question_info.body,
            with_chooses: question_info.with_chooses,
        })
        .get_result::<Question>(connection)
    {
        Ok(ok) => new_question = ok,
        Err(err) => return Err(err),
    };

    let question_answers: Vec<Answer>;
    match insert_into(answers::table)
        .values::<Vec<NewAnswer>>(
            answers_body
                .iter()
                .map(|answer_body| NewAnswer {
                    question_id: new_question.id,
                    body: answer_body.to_string(),
                })
                .collect(),
        )
        .get_results::<Answer>(connection)
    {
        Ok(ok) => question_answers = ok,
        Err(err) => return Err(err),
    };

    let result = QuestionWithAnswers {
        id: new_question.id,
        system_id: new_question.system_id,
        body: new_question.body,
        with_chooses: new_question.with_chooses,
        answers: question_answers,
    };
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
) -> Result<Vec<Question>, Error> {
    match questions_info
        .iter()
        .map(|question_raw| {
            update(questions.find(question_raw.id))
                .set::<UpdateQuestion>(question_raw.clone())
                .get_result::<Question>(connection)
        })
        .collect()
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
