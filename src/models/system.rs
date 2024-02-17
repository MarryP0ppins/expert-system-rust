use crate::schema::systems;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(table_name=systems)]
pub struct System {
    pub id: i32,
    pub user_id: i32,
    pub about: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub private: bool,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=systems)]
pub struct NewSystem {
    pub user_id: i32,
    pub about: Option<String>,
    pub name: String,
    pub private: bool,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name=systems)]
pub struct UpdateSystem {
    pub user_id: Option<i32>,
    pub about: Option<String>,
    pub name: Option<String>,
    pub private: Option<bool>,
}
