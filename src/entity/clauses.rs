//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::Operatorenum;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, IntoActiveModel, Set, Unchanged};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "clauses")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: String,
    pub operator: Operatorenum,
    pub question_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateClauseModel {
    pub id: i32,
    pub compared_value: Option<String>,
    pub logical_group: Option<String>,
    pub operator: Option<Operatorenum>,
    pub question_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NewClauseWithoutRule {
    pub compared_value: String,
    pub logical_group: String,
    pub operator: Operatorenum,
    pub question_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::questions::Entity",
        from = "Column::QuestionId",
        to = "super::questions::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Questions,
    #[sea_orm(
        belongs_to = "super::rules::Entity",
        from = "Column::RuleId",
        to = "super::rules::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Rules,
}

impl Related<super::questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl Related<super::rules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rules.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UpdateClauseModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            compared_value: self
                .compared_value
                .map_or(NotSet, |compared_value| Set(compared_value)),
            logical_group: self
                .logical_group
                .map_or(NotSet, |logical_group| Set(logical_group)),
            operator: self.operator.map_or(NotSet, |operator| Set(operator)),
            question_id: self
                .question_id
                .map_or(NotSet, |question_id| Set(question_id)),
            ..Default::default()
        }
    }
}
