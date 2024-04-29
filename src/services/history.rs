use crate::{
    models::history::{HistoryWithSystem, NewHistory},
    schema::{histories::dsl::*, systems, users},
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_histories(
    connection: &mut AsyncPgConnection,
    _system: Option<i32>,
    _user: Option<i32>,
) -> Result<Vec<HistoryWithSystem>, Error> {
    let mut query = histories
        .inner_join(systems::table)
        .inner_join(users::table)
        .into_boxed();

    if let Some(param) = _system {
        query = query.filter(systems::id.eq(param));
    }
    if let Some(param) = _user {
        query = query.or_filter(users::id.eq(param));
    }

    Ok(query
        .select((
            id,
            (systems::all_columns),
            answered_questions,
            results,
            started_at,
            finished_at,
        ))
        .load::<HistoryWithSystem>(connection)
        .await?)
}

pub async fn create_history(
    connection: &mut AsyncPgConnection,
    history_info: NewHistory,
) -> Result<HistoryWithSystem, Error> {
    let insert_raw_id = insert_into(histories)
        .values::<NewHistory>(history_info)
        .returning(id)
        .get_result::<i32>(connection)
        .await?;

    Ok(histories
        .find(insert_raw_id)
        .inner_join(systems::table)
        .select((
            id,
            (systems::all_columns),
            answered_questions,
            results,
            started_at,
            finished_at,
        ))
        .first::<HistoryWithSystem>(connection)
        .await?)
}

pub async fn delete_history(
    connection: &mut AsyncPgConnection,
    history_id: i32,
) -> Result<usize, Error> {
    match delete(histories.find(history_id)).execute(connection).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
