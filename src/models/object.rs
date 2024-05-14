use crate::schema::objects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    object_attribute_attributevalue::{
        NewObjectAttributeAttributevalueWithoutObject, ObjectAttributeAttributevalue,
    },
    system::System,
};

#[derive(Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable, Debug)]
#[diesel(belongs_to(System))]
#[diesel(table_name=objects)]
pub struct Object {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name=objects)]
pub struct NewObject {
    pub system_id: i32,
    pub name: String,
}

#[derive(Queryable, Deserialize, ToSchema)]
pub struct NewObjectWithAttributesValueIds {
    pub system_id: i32,
    pub name: String,
    pub obeject_attribute_attributevalue_ids: Vec<NewObjectAttributeAttributevalueWithoutObject>,
}

#[derive(Serialize, Deserialize, Queryable, ToSchema, Clone)]
pub struct ObjectWithAttributesValues {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub obeject_attribute_attributevalue_ids: Vec<ObjectAttributeAttributevalue>,
}

#[derive(Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=objects)]
pub struct UpdateObject {
    pub id: i32,
    pub name: String,
}
