use super::system::System;
use chrono::NaiveDateTime;
use serde_json::Value;

pub struct NewHistory {
    pub system_id: i32,
    pub user_id: i32,
    pub answered_questions: String,
    //#[schema(value_type=HashMap<String, u8>)]
    pub results: Value,
}

pub struct HistoryWithSystem {
    pub id: i32,
    pub system: System,
    pub answered_questions: String,
   // #[schema(value_type=HashMap<String, u8>)]
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finished_at: NaiveDateTime,
}
