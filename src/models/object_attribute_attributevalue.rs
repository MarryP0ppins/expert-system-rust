use crate::schema::object_attribute_attributevalue;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::attribute::Attribute;
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
#[diesel(belongs_to(Attribute))]
#[diesel(table_name=object_attribute_attributevalue)]
pub struct ObjectAttributeAttributevalue {
    pub id: i32,
    pub object_id: i32,
    pub attribute_value_id: i32,
    pub attribute_id: i32,
}

#[derive(Queryable, Deserialize, Insertable, Clone, ToSchema, Serialize)]
#[diesel(table_name=object_attribute_attributevalue)]
pub struct NewObjectAttributeAttributevalue {
    pub object_id: i32,
    pub attribute_id: i32,
    pub attribute_value_id: i32,
}

#[derive(Queryable, Deserialize, Insertable, Clone, ToSchema, Serialize)]
#[diesel(table_name=object_attribute_attributevalue)]
pub struct NewObjectAttributeAttributevalueWithoutObject {
    pub attribute_id: i32,
    pub attribute_value_id: i32,
}
