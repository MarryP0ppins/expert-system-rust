//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use super::systems;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq, ToSchema)]
#[sea_orm(table_name = "histories")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub system_id: i32,
    pub user_id: i32,
    pub answered_questions: String,
    pub results: Json,
    pub started_at: DateTime,
    pub finished_at: DateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HistoryWithSystem {
    pub id: i32,
    pub system: systems::Model,
    pub answered_questions: String,
    #[schema(value_type=HashMap<String, u8>)]
    pub results: Value,
    pub started_at: NaiveDateTime,
    pub finished_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::systems::Entity",
        from = "Column::SystemId",
        to = "super::systems::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Systems,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::systems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Systems.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
