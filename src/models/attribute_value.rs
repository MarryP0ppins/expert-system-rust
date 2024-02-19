use crate::schema::attributesvalues;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::attribute::Attribute;

#[derive(Debug, Queryable, Serialize, Identifiable, Associations, Selectable, ToSchema)]
#[diesel(belongs_to(Attribute))]
#[diesel(table_name=attributesvalues)]
pub struct AttributeValue {
    pub id: i32,
    pub attribute_id: i32,
    pub value: String,
}

#[derive(Debug, Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=attributesvalues)]
pub struct NewAttributeValue {
    pub attribute_id: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=attributesvalues)]
pub struct UpdateAttributeValue {
    pub id: i32,
    pub value: String,
}
