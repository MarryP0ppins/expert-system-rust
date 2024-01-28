use super::{system::System, user::User};
use crate::schema::histories;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(table_name=histories)]
pub struct History {
    pub id: i32,
    pub system: i32,
    pub user: i32,
    pub answered_questions: String,
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finish_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name=histories)]
pub struct NewHistory {
    pub system: i32,
    pub user: i32,
    pub answered_questions: String,
    pub results: Value,
}

#[derive(Debug, Queryable, Serialize)]
pub struct HistoryWithSystemAndUser {
    pub id: i32,
    pub system: System,
    pub user: User,
    pub answered_questions: String,
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finish_at: NaiveDateTime,
}
