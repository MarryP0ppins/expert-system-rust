//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "object_attribute_attributevalue")]
pub struct Model {
    pub id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub object_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub attribute_value_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub attribute_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::attributes::Entity",
        from = "Column::AttributeId",
        to = "super::attributes::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Attributes,
    #[sea_orm(
        belongs_to = "super::attributesvalues::Entity",
        from = "Column::AttributeValueId",
        to = "super::attributesvalues::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Attributesvalues,
    #[sea_orm(
        belongs_to = "super::objects::Entity",
        from = "Column::ObjectId",
        to = "super::objects::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Objects,
}

impl Related<super::attributes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attributes.def()
    }
}

impl Related<super::attributesvalues::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attributesvalues.def()
    }
}

impl Related<super::objects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Objects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
