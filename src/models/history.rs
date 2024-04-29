use super::system::System;
use crate::schema::histories;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Queryable, Insertable, Deserialize, Clone, ToSchema)]
#[diesel(table_name=histories)]
pub struct NewHistory {
    pub system_id: i32,
    pub user_id: i32,
    pub answered_questions: String,
    #[schema(value_type=HashMap<String, u8>)]
    pub results: Value,
}

#[derive(Queryable, Serialize, ToSchema, Clone)]
pub struct HistoryWithSystem {
    pub id: i32,
    pub system: System,
    pub answered_questions: String,
    #[schema(value_type=HashMap<String, u8>)]
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finished_at: NaiveDateTime,
}
