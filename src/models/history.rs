use super::{system::System, user::UserWithoutPassword};
use crate::schema::histories;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
/*
#[derive( Queryable, Serialize, Identifiable)]
#[diesel(table_name=histories)]
pub struct History {
    pub id: i32,
    pub system_id: i32,
    pub user_id: i32,
    pub answered_questions: String,
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finish_at: NaiveDateTime,
}
 */

#[derive(Queryable, Insertable, Deserialize, Clone, ToSchema)]
#[diesel(table_name=histories)]
pub struct NewHistory {
    pub system_id: i32,
    pub user_id: i32,
    pub answered_questions: String,
    pub results: Value,
}

#[derive(Queryable, Serialize, ToSchema)]
pub struct HistoryWithSystemAndUser {
    pub id: i32,
    pub system: System,
    pub user: UserWithoutPassword,
    pub answered_questions: String,
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finish_at: NaiveDateTime,
}
