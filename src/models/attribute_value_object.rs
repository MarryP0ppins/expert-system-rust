use crate::schema::attributesvalue_object;
use diesel::prelude::*;

use super::attribute_value::AttributeValue;
use super::object::Object;

#[derive(Debug, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Object))]
#[diesel(belongs_to(AttributeValue))]
#[diesel(table_name=attributesvalue_object)]
pub struct AttributeValueObject {
    pub id: i32,
    pub object_id: i32,
    pub attribute_value_id: i32,
}
