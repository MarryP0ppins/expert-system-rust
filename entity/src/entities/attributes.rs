//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::attributesvalues::AttributeValueModel;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq, ToSchema)]
#[schema(as = AttributeModel)]
#[sea_orm(table_name = "attributes")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    #[schema(read_only)]
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}
pub use Model as AttributeModel;

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
pub struct UpdateAttributeModel {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AttributeWithAttributeValuesModel {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub values: Vec<AttributeValueModel>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NewAttributeWithAttributeValuesModel {
    pub system_id: i32,
    pub name: String,
    pub values_name: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::attributesvalues::Entity")]
    Attributesvalues,
    #[sea_orm(has_many = "super::object_attribute_attributevalue::Entity")]
    ObjectAttributeAttributevalue,
    #[sea_orm(has_many = "super::rule_attribute_attributevalue::Entity")]
    RuleAttributeAttributevalue,
    #[sea_orm(
        belongs_to = "super::systems::Entity",
        from = "Column::SystemId",
        to = "super::systems::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Systems,
}

impl Related<super::attributesvalues::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attributesvalues.def()
    }
}

impl Related<super::object_attribute_attributevalue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ObjectAttributeAttributevalue.def()
    }
}

impl Related<super::rule_attribute_attributevalue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RuleAttributeAttributevalue.def()
    }
}

impl Related<super::systems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Systems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
