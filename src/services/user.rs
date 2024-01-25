use crate::{
    models::user::{NewUser, User},
    schema::users::dsl::*,
};
use diesel::{insert_into, prelude::*, result::Error};
use rocket::serde::json::Json;

pub fn get_users(connection: &mut PgConnection) -> Result<Vec<User>, Error> {
    let result = users.load::<User>(connection);

    match result {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}

pub fn create_user(
    connection: &mut PgConnection,
    user_info: Json<NewUser>,
) -> Result<Vec<User>, Error> {
    let new_system = NewUser {
        email: user_info.email.clone(),
        username: user_info.username.clone(),
        first_name: user_info.first_name.clone(),
        last_name: user_info.last_name.clone(),
        is_superuser: user_info.is_superuser,
        password: user_info.password.clone(),
    };

    let result = insert_into(users)
        .values::<NewUser>(new_system)
        .get_results(connection);

    match result {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}
