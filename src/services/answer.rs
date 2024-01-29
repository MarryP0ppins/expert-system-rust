use crate::{
    models::answer::{Answer, NewAnswer, UpdateAnswer},
    schema::answers::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};

pub fn get_answers(connection: &mut PgConnection, question: i32) -> Result<Vec<Answer>, Error> {
    match answers
        .filter(question_id.eq(question))
        .load::<Answer>(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_answer(
    connection: &mut PgConnection,
    answer_info: NewAnswer,
) -> Result<Answer, Error> {
    match insert_into(answers)
        .values::<NewAnswer>(answer_info)
        .get_result::<Answer>(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_delete_answers(
    connection: &mut PgConnection,
    answers_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(answers.filter(id.eq_any(answers_ids))).execute(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_update_answers(
    connection: &mut PgConnection,
    answer_info: Vec<UpdateAnswer>,
) -> Result<Vec<Answer>, Error> {
    match answer_info
        .iter()
        .map(|answer_raw| {
            update(answers.find(answer_raw.id))
                .set::<UpdateAnswer>(answer_raw.clone())
                .get_result::<Answer>(connection)
        })
        .collect()
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
