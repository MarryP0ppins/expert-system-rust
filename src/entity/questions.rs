//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::{entity::prelude::*, ActiveValue::NotSet, IntoActiveModel, Set, Unchanged};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::answers;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "questions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateQuestionModel {
    pub id: i32,
    pub body: Option<String>,
    pub with_chooses: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionWithAnswersModel {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers: Vec<answers::Model>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NewQuestionWithAnswersModel {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers_body: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::answers::Entity")]
    Answers,
    #[sea_orm(has_many = "super::clauses::Entity")]
    Clauses,
    #[sea_orm(has_many = "super::rule_question_answer::Entity")]
    RuleQuestionAnswer,
    #[sea_orm(
        belongs_to = "super::systems::Entity",
        from = "Column::SystemId",
        to = "super::systems::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Systems,
}

impl Related<super::answers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Answers.def()
    }
}

impl Related<super::clauses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clauses.def()
    }
}

impl Related<super::rule_question_answer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RuleQuestionAnswer.def()
    }
}

impl Related<super::systems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Systems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UpdateQuestionModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            body: self.body.map_or(NotSet, |body| Set(body)),
            with_chooses: self
                .with_chooses
                .map_or(NotSet, |with_chooses| Set(with_chooses)),
            ..Default::default()
        }
    }
}
