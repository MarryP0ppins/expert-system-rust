use crate::schema::questions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::answer::Answer;

#[derive(Queryable, Serialize, Deserialize, Identifiable, Selectable, Clone, Debug)]
#[diesel(table_name=questions)]
pub struct Question {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name=questions)]
pub struct NewQuestion {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Queryable, Deserialize, ToSchema)]
pub struct NewQuestionWithAnswersBody {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers_body: Vec<String>,
}

#[derive(Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=questions)]
pub struct UpdateQuestion {
    pub id: i32,
    pub body: Option<String>,
    pub with_chooses: Option<bool>,
}

#[derive(Queryable, Serialize, Clone, ToSchema, Debug)]
pub struct QuestionWithAnswers {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers: Vec<Answer>,
}
