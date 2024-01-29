use crate::{
    models::{
        history::{History, HistoryWithSystemAndUser, NewHistory},
        system::System,
        user::User,
    },
    schema::{histories::dsl::*, systems, users},
};
use diesel::{delete, insert_into, prelude::*, result::Error};

pub fn get_histories(
    connection: &mut PgConnection,
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
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_history(
    connection: &mut PgConnection,
    history_info: NewHistory,
) -> Result<HistoryWithSystemAndUser, Error> {
    let raw: History;

    match insert_into(histories)
        .values::<NewHistory>(history_info.clone())
        .get_result::<History>(connection)
    {
        Ok(ok) => raw = ok,
        Err(err) => return Err(err),
    };

    let history_system: System;
    match systems::table
        .find(history_info.system_id)
        .first::<System>(connection)
    {
        Ok(ok) => history_system = ok,
        Err(err) => return Err(err),
    };

    let history_user: User;
    match users::table
        .find(history_info.user_id)
        .first::<User>(connection)
    {
        Ok(ok) => history_user = ok,
        Err(err) => return Err(err),
    };

    let result = HistoryWithSystemAndUser {
        id: raw.id,
        system_id: history_system,
        user_id: history_user,
        answered_questions: raw.answered_questions,
        results: raw.results,
        started_at: raw.started_at,
        finish_at: raw.finish_at,
    };
    Ok(result)
}

pub fn delete_history(connection: &mut PgConnection, history_id: i32) -> Result<usize, Error> {
    match delete(histories.find(history_id)).execute(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
