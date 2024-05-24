//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "objects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub system_id: i32,
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