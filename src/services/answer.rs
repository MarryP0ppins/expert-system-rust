use crate::{
    models::answer::{Answer, NewAnswer, UpdateAnswer},
    schema::answers::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};

pub fn get_answers(connection: &mut PgConnection, _question: i32) -> Result<Vec<Answer>, Error> {
    let result = answers
        .filter(question.eq(_question))
        .load::<Answer>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_answer(
    connection: &mut PgConnection,
    answer_info: NewAnswer,
) -> Result<Answer, Error> {
    let result = insert_into(answers)
        .values::<NewAnswer>(answer_info)
        .get_result::<Answer>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_delete_answers(
    connection: &mut PgConnection,
    answers_ids: Vec<i32>,
) -> Result<usize, Error> {
    let result = delete(answers.filter(id.eq_any(answers_ids))).execute(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_update_answers(
    connection: &mut PgConnection,
    answer_info: Vec<UpdateAnswer>,
) -> Result<Vec<Answer>, Error> {
    let result = answer_info
        .iter()
        .map(|answer_raw| {
            update(answers.find(answer_raw.id))
                .set::<UpdateAnswer>(answer_raw.clone())
                .get_result::<Answer>(connection)
        })
        .collect();

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
