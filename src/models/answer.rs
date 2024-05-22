use crate::schema::answers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::question::Question;

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Identifiable,
    Associations,
    Selectable,
    Clone,
    ToSchema,
    Debug,
)]
#[diesel(belongs_to(Question))]
#[diesel(table_name=answers)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub body: String,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=answers)]
pub struct NewAnswer {
    pub question_id: i32,
    pub body: String,
}

#[derive(Deserialize, AsChangeset, Clone, ToSchema, Debug)]
#[diesel(table_name=answers)]
pub struct UpdateAnswer {
    pub id: i32,
    pub body: String,
}
