use crate::{
    models::system::{NewSystem, System, UpdateSystem},
    schema::systems::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};

pub fn get_systems(
    connection: &mut PgConnection,
    _name: Option<String>,
    _user_id: Option<i32>,
) -> Result<Vec<System>, Error> {
    let mut query = systems.into_boxed();

    if let Some(param) = _name {
        query = query.filter(name.like(format!("%{}%", param)));
    }

    if let Some(_user_id) = _user_id {
        query = query.filter(user_id.eq(_user_id));
    }

    match query.load::<System>(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn get_system(connection: &mut PgConnection, system_id: i32) -> Result<System, Error> {
    match systems.find(system_id).first::<System>(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_system(
    connection: &mut PgConnection,
    system_info: NewSystem,
) -> Result<System, Error> {
    match insert_into(systems)
        .values::<NewSystem>(system_info)
        .get_result::<System>(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn update_system(
    connection: &mut PgConnection,
    system_id: i32,
    system_info: UpdateSystem,
) -> Result<System, Error> {
    match update(systems.find(system_id))
        .set::<UpdateSystem>(system_info)
        .get_result::<System>(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn delete_system(connection: &mut PgConnection, system_id: i32) -> Result<usize, Error> {
    match delete(systems.find(system_id)).execute(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
