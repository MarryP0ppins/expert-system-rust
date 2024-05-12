use crate::schema::attributesvalue_object;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::attribute_value::AttributeValue;
use super::object::Object;

#[derive(
    Queryable,
    Identifiable,
    Associations,
    Selectable,
    Serialize,
    Deserialize,
    Clone,
    ToSchema,
    Debug,
)]
#[diesel(belongs_to(Object))]
#[diesel(belongs_to(AttributeValue))]
#[diesel(table_name=attributesvalue_object)]
pub struct AttributeValueObject {
    pub id: i32,
    pub object_id: i32,
    pub attribute_value_id: i32,
    pub attribute_id: i32,
}

#[derive(Queryable, Deserialize, Insertable, Clone, ToSchema, Serialize)]
#[diesel(table_name=attributesvalue_object)]
pub struct NewAttributeValueObject {
    pub object_id: i32,
    pub attribute_id: i32,
    pub attribute_value_id: i32,
}
