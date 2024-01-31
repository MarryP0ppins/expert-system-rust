use crate::schema::attributesvalues;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::attribute::Attribute;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Attribute))]
#[diesel(table_name=attributesvalues)]
pub struct AttributeValue {
    pub id: i32,
    pub attribute_id: i32,
    pub value: String,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=attributesvalues)]
pub struct NewAttributeValue {
    pub attribute_id: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name=attributesvalues)]
pub struct UpdateAttributeValue {
    pub id: i32,
    pub value: String,
}
