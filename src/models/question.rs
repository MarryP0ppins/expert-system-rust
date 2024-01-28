use crate::schema::questions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::answer::Answer;

#[derive(Debug, Queryable, Serialize, Identifiable, Selectable, Clone)]
#[diesel(table_name=questions)]
pub struct Question {
    pub id: i32,
    pub system: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=questions)]
pub struct NewQuestion {
    pub system: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name=questions)]
pub struct NewQuestionWithAnswersBody {
    pub system: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers_body: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name=questions)]
pub struct UpdateQuestion {
    pub id: i32,
    pub body: Option<String>,
    pub with_chooses: Option<bool>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct QuestionWithAnswers {
    pub id: i32,
    pub system: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers: Vec<Answer>,
}