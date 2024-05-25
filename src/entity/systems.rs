//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    answers, attributes, attributesvalues, clauses, object_attribute_attributevalue, objects,
    questions, rule_attribute_attributevalue, rule_question_answer, rules,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq, ToSchema)]
#[sea_orm(table_name = "systems")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub about: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: DateTime,
    #[serde(skip_deserializing)]
    pub updated_at: DateTime,
    #[sea_orm(unique)]
    pub name: String,
    pub private: bool,
    pub image_uri: Option<String>,
}

#[derive(ToSchema, TryFromMultipart)]
pub struct NewSystemMultipartModel {
    pub about: Option<String>,
    pub name: String,
    #[schema(value_type = String, format = Binary)]
    #[form_data(limit = "1MiB")]
    pub image: Option<FieldData<Bytes>>,
    pub private: bool,
}

#[derive(Serialize, Clone)]
pub struct SystemsWithPageCount {
    pub systems: Vec<Model>,
    pub pages: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemModel {
    pub about: Option<String>,
    pub name: Option<String>,
    pub image_uri: Option<String>,
    pub private: Option<bool>,
}

#[derive(ToSchema, TryFromMultipart, Debug)]
pub struct UpdateSystemMultipartModel {
    pub about: Option<String>,
    pub name: Option<String>,
    #[schema(value_type = Option<String>, format = Binary)]
    #[form_data(limit = "1MiB")]
    pub image: Option<FieldData<Bytes>>,
    pub private: Option<bool>,
    pub is_image_removed: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct SystemDeleteModel {
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct SystemBackupModel {
    pub system: Model, //
    pub objects: Vec<objects::Model>,
    pub object_attribute_attributevalue: Vec<object_attribute_attributevalue::Model>,
    pub attributes: Vec<attributes::Model>,              //
    pub attributes_values: Vec<attributesvalues::Model>, //
    pub rules: Vec<rules::Model>,
    pub rule_attribute_attributevalue: Vec<rule_attribute_attributevalue::Model>,
    pub clauses: Vec<clauses::Model>,
    pub questions: Vec<questions::Model>, //
    pub answers: Vec<answers::Model>,     //
    pub rule_question_answer: Vec<rule_question_answer::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::attributes::Entity")]
    Attributes,
    #[sea_orm(has_many = "super::histories::Entity")]
    Histories,
    #[sea_orm(has_many = "super::objects::Entity")]
    Objects,
    #[sea_orm(has_many = "super::questions::Entity")]
    Questions,
    #[sea_orm(has_many = "super::rules::Entity")]
    Rules,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::attributes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attributes.def()
    }
}

impl Related<super::histories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Histories.def()
    }
}

impl Related<super::objects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Objects.def()
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

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UpdateSystemModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            about: Set(self.about),
            name: self.name.map_or(NotSet, |name| Set(name)),
            image_uri: Set(self.image_uri),
            private: self.private.map_or(NotSet, |private| Set(private)),
            ..Default::default()
        }
    }
}
