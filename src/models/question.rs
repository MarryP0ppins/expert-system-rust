use crate::schema::questions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::answer::Answer;

#[derive(Debug, Queryable, Serialize, Identifiable, Selectable, Clone)]
#[diesel(table_name=questions)]
pub struct Question {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=questions)]
pub struct NewQuestion {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Debug, Queryable, Deserialize, ToSchema)]
pub struct NewQuestionWithAnswersBody {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers_body: Vec<String>,
}

#[derive(Debug, Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=questions)]
pub struct UpdateQuestion {
    pub id: i32,
    pub body: Option<String>,
    pub with_chooses: Option<bool>,
}

#[derive(Debug, Queryable, Serialize, Clone, ToSchema)]
pub struct QuestionWithAnswers {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers: Vec<Answer>,
}
