use crate::{
    models::answer::{Answer, NewAnswer, UpdateAnswer},
    schema::answers::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_answers(
    connection: &mut AsyncPgConnection,
    question: i32,
) -> Result<Vec<Answer>, Error> {
    Ok(answers
        .filter(question_id.eq(question))
        .load::<Answer>(connection)
        .await?)
}

pub async fn create_answer(
    connection: &mut AsyncPgConnection,
    answer_info: Vec<NewAnswer>,
) -> Result<Vec<Answer>, Error> {
    Ok(insert_into(answers)
        .values::<Vec<NewAnswer>>(answer_info)
        .get_results::<Answer>(connection)
        .await?)
}

pub async fn multiple_delete_answers(
    connection: &mut AsyncPgConnection,
    answers_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(answers.filter(id.eq_any(answers_ids)))
        .execute(connection)
        .await?)
}

pub async fn multiple_update_answers(
    connection: &mut AsyncPgConnection,
    answer_info: Vec<UpdateAnswer>,
) -> Result<Vec<Answer>, Error> {
    let mut _answers: Vec<Answer> = vec![];

    for answer_raw in answer_info.into_iter() {
        match update(answers.find(answer_raw.id))
            .set::<UpdateAnswer>(answer_raw)
            .get_result::<Answer>(connection)
            .await
        {
            Ok(result) => _answers.push(result),
            Err(err) => return Err(err),
        }
    }

    //сделать выполнение всех апдейтов даже если есть ошибка в одном обновлении
    Ok(_answers)
}
