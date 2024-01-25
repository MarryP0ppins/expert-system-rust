use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
/*
* User models begin from here
*/

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_name: String,
    pub is_superuser: bool,
    pub password: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_superuser: bool,
    pub password: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name=users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_superuser: Option<bool>,
    pub password: Option<String>,
}
