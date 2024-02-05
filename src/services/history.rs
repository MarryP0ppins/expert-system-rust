use crate::{
    models::history::{HistoryWithSystemAndUser, NewHistory},
    schema::{histories::dsl::*, systems, users},
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_histories(
    connection: &mut AsyncPgConnection,
    _system: Option<i32>,
    _user: Option<i32>,
) -> Result<Vec<HistoryWithSystemAndUser>, Error> {
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

    match query
        .select((
            id,
            (systems::all_columns),
            (users::all_columns),
            answered_questions,
            results,
            started_at,
            finished_at,
        ))
        .load::<HistoryWithSystemAndUser>(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn create_history(
    connection: &mut AsyncPgConnection,
    history_info: NewHistory,
) -> Result<HistoryWithSystemAndUser, Error> {
    let insert_raw_id: i32;

    match insert_into(histories)
        .values::<NewHistory>(history_info.clone())
        .returning(id)
        .get_result(connection)
        .await
    {
        Ok(ok) => insert_raw_id = ok,
        Err(err) => return Err(err),
    };

    match histories
        .find(insert_raw_id)
        .inner_join(systems::table)
        .inner_join(users::table)
        .select((
            id,
            (systems::all_columns),
            (users::all_columns),
            answered_questions,
            results,
            started_at,
            finished_at,
        ))
        .first::<HistoryWithSystemAndUser>(connection)
        .await
    {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err),
    }
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
