use crate::schema::attributes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{attribute_value::AttributeValue, system::System};

#[derive(
    Queryable, Serialize, Deserialize, Identifiable, Selectable, Associations, Clone, Debug,
)]
#[diesel(belongs_to(System))]
#[diesel(table_name=attributes)]
pub struct Attribute {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name=attributes)]
pub struct NewAttribute {
    pub system_id: i32,
    pub name: String,
}

#[derive(Queryable, Deserialize, ToSchema)]
pub struct NewAttributeWithAttributeValuesName {
    pub system_id: i32,
    pub name: String,
    pub values_name: Vec<String>,
}

#[derive(Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=attributes)]
pub struct UpdateAttribute {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, ToSchema, Clone)]
pub struct AttributeWithAttributeValues {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub values: Vec<AttributeValue>,
}
