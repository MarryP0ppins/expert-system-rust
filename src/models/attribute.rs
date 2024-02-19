use crate::schema::attributes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::attribute_value::AttributeValue;

#[derive(Debug, Queryable, Serialize, Identifiable, Selectable, Clone)]
#[diesel(table_name=attributes)]
pub struct Attribute {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=attributes)]
pub struct NewAttribute {
    pub system_id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Deserialize, ToSchema)]
pub struct NewAttributeWithAttributeValuesName {
    pub system_id: i32,
    pub name: String,
    pub values_name: Vec<String>,
}

#[derive(Debug, Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=attributes)]
pub struct UpdateAttribute {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Serialize, ToSchema)]
pub struct AttributeWithAttributeValues {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub values: Vec<AttributeValue>,
}
