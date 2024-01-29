use crate::{
    models::user::{NewUser, User},
    schema::users::dsl::*,
};
use diesel::{insert_into, prelude::*, result::Error};

pub fn get_users(connection: &mut PgConnection) -> Result<Vec<User>, Error> {
    match users.load::<User>(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn get_user(connection: &mut PgConnection, user_id: i32) -> Result<User, Error> {
    match users.find(user_id).first::<User>(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_user(connection: &mut PgConnection, user_info: NewUser) -> Result<User, Error> {
    match insert_into(users)
        .values::<NewUser>(user_info)
        .get_result(connection)
    {
        Ok(system) => Ok(system),
        Err(err) => Err(err),
    }
}
