use crate::{
    models::{
        history::{History, HistoryWithSystemAndUser, NewHistory},
        system::System,
        user::User,
    },
    schema::{histories::dsl::*, systems, users},
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use rocket::serde::json::Json;

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

    let result = query
        .select((
            id,
            (systems::all_columns),
            (users::all_columns),
            answered_questions,
            results,
            started_at,
            finished_at,
        ))
        .load::<HistoryWithSystemAndUser>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_history(
    connection: &mut PgConnection,
    history_info: Json<NewHistory>,
) -> Result<HistoryWithSystemAndUser, Error> {
    let new_history = NewHistory {
        answered_questions: history_info.answered_questions.to_owned(),
        results: history_info.results.to_owned(),
        ..history_info.0
    };

    let raw = insert_into(histories)
        .values::<NewHistory>(new_history.clone())
        .get_result::<History>(connection);

    match raw {
        Ok(raw) => {
            let history_system = systems::table
                .find(new_history.system)
                .first::<System>(connection)?;
            let history_user = users::table
                .find(new_history.user)
                .first::<User>(connection)?;

            let result = HistoryWithSystemAndUser {
                id: raw.id,
                system: history_system,
                user: history_user,
                answered_questions: raw.answered_questions,
                results: raw.results,
                started_at: raw.started_at,
                finish_at: raw.finish_at,
            };
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub fn delete_history(connection: &mut PgConnection, history_id: i32) -> Result<usize, Error> {
    let result = delete(histories)
        .filter(id.eq(history_id))
        .execute(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
