use crate::{
    models::user::{NewUser, User},
    schema::users::dsl::*,
};
use diesel::{insert_into, prelude::*, result::Error};

pub fn get_users(connection: &mut PgConnection) -> Result<Vec<User>, Error> {
    let result = users.load::<User>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn get_user(connection: &mut PgConnection, user_id: i32) -> Result<User, Error> {
    let result = users.find(user_id).first::<User>(connection);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_user(connection: &mut PgConnection, user_info: NewUser) -> Result<User, Error> {
    let result = insert_into(users)
        .values::<NewUser>(user_info)
        .get_result(connection);

    match result {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}
