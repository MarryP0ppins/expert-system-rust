use crate::{
    models::clause::{Clause, NewClause, UpdateClause},
    schema::clauses::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_clauses(
    connection: &mut AsyncPgConnection,
    rule: i32,
) -> Result<Vec<Clause>, Error> {
    Ok(clauses
        .filter(rule_id.eq(rule))
        .load::<Clause>(connection)
        .await?)
}

pub async fn create_clauses(
    connection: &mut AsyncPgConnection,
    clause_info: Vec<NewClause>,
) -> Result<Vec<Clause>, Error> {
    Ok(insert_into(clauses)
        .values::<Vec<NewClause>>(clause_info)
        .get_results::<Clause>(connection)
        .await?)
}

pub async fn multiple_delete_clauses(
    connection: &mut AsyncPgConnection,
    clauses_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(clauses.filter(id.eq_any(clauses_ids)))
        .execute(connection)
        .await?)
}

pub async fn multiple_update_clauses(
    connection: &mut AsyncPgConnection,
    clauses_info: Vec<UpdateClause>,
) -> Result<Vec<Clause>, Error> {
    let mut _clauses: Vec<Clause> = vec![];

    for clause_raw in clauses_info.into_iter() {
        match update(clauses.find(clause_raw.id))
            .set::<UpdateClause>(clause_raw)
            .get_result::<Clause>(connection)
            .await
        {
            Ok(result) => _clauses.push(result),
            Err(err) => return Err(err),
        }
    }

    Ok(_clauses)
}
