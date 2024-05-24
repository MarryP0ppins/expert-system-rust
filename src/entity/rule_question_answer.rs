//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "rule_question_answer")]
pub struct Model {
    pub id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub answer_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub rule_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub question_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::answers::Entity",
        from = "Column::AnswerId",
        to = "super::answers::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Answers,
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

impl Related<super::answers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Answers.def()
    }
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