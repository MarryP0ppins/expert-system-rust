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
    _system: i32,
) -> Result<Vec<QuestionWithAnswers>, Error> {
    let questions_filtered: Result<Vec<Question>, Error> = questions
        .filter(system.eq(_system))
        .load::<Question>(connection);

    match questions_filtered {
        Ok(questions_filtered) => {
            let answers_belonging: Vec<Answer> =
                Answer::belonging_to(&questions_filtered).load::<Answer>(connection)?;

            let result = answers_belonging
                .grouped_by(&questions_filtered)
                .into_iter()
                .zip(questions_filtered)
                .map(|(answers, _question)| QuestionWithAnswers {
                    id: _question.id,
                    answers,
                    system: _question.system,
                    body: _question.body,
                    with_chooses: _question.with_chooses,
                })
                .collect::<Vec<QuestionWithAnswers>>();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub fn create_question(
    connection: &mut PgConnection,
    question_info: NewQuestionWithAnswersBody,
) -> Result<QuestionWithAnswers, Error> {
    let answers_body = question_info.answers_body;
    let new_question = insert_into(questions)
        .values::<NewQuestion>(NewQuestion {
            system: question_info.system,
            body: question_info.body,
            with_chooses: question_info.with_chooses,
        })
        .get_result::<Question>(connection);

    match new_question {
        Ok(new_question) => {
            let mut question_answers: Vec<Answer> = vec![];
            if let Some(values) = answers_body {
                question_answers = values
                    .iter()
                    .map(|answer_body| {
                        insert_into(answers::table)
                            .values::<NewAnswer>(NewAnswer {
                                question: new_question.id,
                                body: answer_body.to_string(),
                            })
                            .get_result::<Answer>(connection)
                            .expect("Answer create failure")
                    })
                    .collect()
            }

            let result = QuestionWithAnswers {
                id: new_question.id,
                system: new_question.system,
                body: new_question.body,
                with_chooses: new_question.with_chooses,
                answers: question_answers,
            };
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub fn multiple_delete_questions(
    connection: &mut PgConnection,
    questions_ids: Vec<i32>,
) -> Result<usize, Error> {
    let result = delete(questions.filter(id.eq_any(questions_ids))).execute(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_update_questions(
    connection: &mut PgConnection,
    questions_info: Vec<UpdateQuestion>,
) -> Result<Vec<Question>, Error> {
    let result = questions_info
        .iter()
        .map(|question_raw| {
            update(questions.find(question_raw.id))
                .set::<UpdateQuestion>(question_raw.clone())
                .get_result::<Question>(connection)
        })
        .collect();

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
