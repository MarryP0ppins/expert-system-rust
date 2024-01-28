use crate::{
    models::system::{NewSystem, System, UpdateSystem},
    schema::systems::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};

pub fn get_systems(
    connection: &mut PgConnection,
    _name: Option<String>,
) -> Result<Vec<System>, Error> {
    let mut query = systems.into_boxed();

    if let Some(param) = _name {
        query = query.filter(name.like(format!("%{}%", param)));
    }

    let result = query.load::<System>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn get_system(connection: &mut PgConnection, system_id: i32) -> Result<System, Error> {
    let result = systems.find(system_id).first::<System>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_system(
    connection: &mut PgConnection,
    system_info: NewSystem,
) -> Result<System, Error> {
    let result = insert_into(systems)
        .values::<NewSystem>(system_info)
        .get_result::<System>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn update_system(
    connection: &mut PgConnection,
    system_id: i32,
    system_info: UpdateSystem,
) -> Result<System, Error> {
    let result = update(systems.find(system_id))
        .set::<UpdateSystem>(system_info)
        .get_result::<System>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn delete_system(connection: &mut PgConnection, system_id: i32) -> Result<usize, Error> {
    let result = delete(systems.find(system_id)).execute(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
