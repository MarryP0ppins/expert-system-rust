use crate::schema::answers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
/*
* User models begin from here
*/

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(table_name=answers)]
pub struct Answer {
    pub id: i32,
    pub question: i32,
    pub body: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=answers)]
pub struct NewAnswer {
    pub question: i32,
    pub body: String,
}

#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name=answers)]
pub struct UpdateAnswer {
    pub id: i32,
    pub body: String,
}
