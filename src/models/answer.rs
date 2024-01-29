use crate::schema::answers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::question::Question;

#[derive(Debug, Queryable, Serialize, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Question))]
#[diesel(table_name=answers)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub body: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=answers)]
pub struct NewAnswer {
    pub question_id: i32,
    pub body: String,
}

#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name=answers)]
pub struct UpdateAnswer {
    pub id: i32,
    pub body: String,
}
