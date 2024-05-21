use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    models::answer::{Answer, NewAnswer, UpdateAnswer},
    schema::answers::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use futures::{
    future::{join_all, try_join_all},
    try_join,
};
use tokio::{join, runtime::Runtime, task};
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

// pub async fn multiple_update_answers(
//     connection: &mut AsyncPgConnection,
//     answer_info: Vec<UpdateAnswer>,
// ) -> Result<Vec<Answer>, Error> {
//     let rt = Runtime::new().unwrap();
//     let mut tasks = vec![];
//     let shared_connection = Arc::new(Mutex::new(connection));
//     rt.block_on(async {
//         for answer_raw in answer_info.into_iter() {
//             let shared = Arc::clone(&shared_connection);
//             let join_handle: thread::JoinHandle<Answer> = thread::spawn(move || {
//                 // some work here
//                 let con = *shared.lock().unwrap();
//                 update(answers.find(answer_raw.id))
//                     .set::<UpdateAnswer>(answer_raw)
//                     .get_result::<Answer>(con)
//             });
//             tasks.push(join_handle)
//         }
//     });

//     let test = tasks.into_iter().map(|task| task.join().unwrap());
//     // Collect all successful updates
//     // let mut updated_answers = vec![];
//     // for result in results {
//     //     match result {
//     //         Ok(answer) => updated_answers.push(answer),
//     //         Err(err) => return Err(err), // Log the error
//     //     }
//     // }
//     // Return all successfully updated answers
//     Ok(vec![Answer {
//         id: 1,
//         question_id: 1,
//         body: "".to_string(),
//     }])
// }

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

// let shared_connection = Arc::new(Mutex::new(connection));

//     for answer_raw in answer_info.into_iter() {
//         let shared_connection_cloned = Arc::clone(&shared_connection);
//         thread::spawn( move|| {
//             let con = shared_connection_cloned.lock().unwrap();
//             update(answers.find(answer_raw.id))
//                 .set::<UpdateAnswer>(answer_raw)
//                 .get_result::<Answer>(con)
//         });
//     }
