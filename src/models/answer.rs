use crate::schema::answers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::question::Question;

#[derive(Debug, Queryable, Serialize, Identifiable, Associations, Selectable, ToSchema)]
#[diesel(belongs_to(Question))]
#[diesel(table_name=answers)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub body: String,
}

#[derive(Debug, Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=answers)]
pub struct NewAnswer {
    pub question_id: i32,
    pub body: String,
}

#[derive(Debug, Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=answers)]
pub struct UpdateAnswer {
    pub id: i32,
    pub body: String,
}
