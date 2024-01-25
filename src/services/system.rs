use crate::{
    models::system::{NewSystem, System},
    schema::systems::dsl::*,
};
use diesel::{insert_into, prelude::*, result::Error};
use rocket::serde::json::Json;

pub fn get_systems(connection: &mut PgConnection) -> Result<Vec<System>, Error> {
    let result = systems.load::<System>(connection);

    match result {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}

pub fn create_system(
    connection: &mut PgConnection,
    system_info: Json<NewSystem>,
) -> Result<Vec<System>, Error> {
    let new_system = NewSystem {
        user: system_info.user,
        about: system_info.about.to_owned(),
        name: system_info.name.to_owned(),
        private: system_info.private,
    };

    let result = insert_into(systems)
        .values::<NewSystem>(new_system)
        .get_results::<System>(connection);

    match result {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}
