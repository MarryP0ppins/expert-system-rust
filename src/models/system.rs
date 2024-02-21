use crate::schema::systems;
use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Queryable, Serialize, Identifiable, ToSchema)]
#[diesel(table_name=systems)]
pub struct System {
    pub id: i32,
    pub user_id: i32,
    pub about: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub private: bool,
    pub image_uri: String,
}

#[derive(Debug, Queryable, ToSchema, TryFromMultipart)]
pub struct NewSystemMultipart {
    pub user_id: i32,
    pub about: Option<String>,
    pub name: String,
    #[schema(value_type = String, format = Binary)]
    #[form_data(limit = "1MiB")]
    pub image: FieldData<Bytes>,
    pub private: bool,
}

#[derive(Debug, Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=systems)]
pub struct NewSystem {
    pub user_id: i32,
    pub about: Option<String>,
    pub name: String,
    pub image_uri: String,
    pub private: bool,
}

#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name=systems)]
pub struct UpdateSystem {
    pub about: Option<String>,
    pub name: Option<String>,
    pub image_uri: Option<String>,
    pub private: Option<bool>,
}
