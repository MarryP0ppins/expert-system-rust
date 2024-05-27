//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::object_attribute_attributevalue::{
    NewObjectAttributeAttributevalueWithoutObjectModel, ObjectAttributeAttributeValueModel,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq, ToSchema)]
#[schema(as = ObjectModel)]
#[sea_orm(table_name = "objects")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    #[schema(read_only)]
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

pub use Model as ObjectModel;

#[derive(Deserialize, ToSchema)]
pub struct NewObjectWithAttributesValueIdsModel {
    pub system_id: i32,
    pub name: String,
    pub object_attribute_attributevalue_ids:
        Vec<NewObjectAttributeAttributevalueWithoutObjectModel>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ObjectWithAttributesValuesModel {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub object_attribute_attributevalue_ids: Vec<ObjectAttributeAttributeValueModel>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
pub struct UpdateObjectModel {
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::object_attribute_attributevalue::Entity")]
    ObjectAttributeAttributevalue,
    #[sea_orm(
        belongs_to = "super::systems::Entity",
        from = "Column::SystemId",
        to = "super::systems::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Systems,
}

impl Related<super::object_attribute_attributevalue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ObjectAttributeAttributevalue.def()
    }
}

impl Related<super::systems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Systems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
