use crate::schema::rule_attribute_attributevalue;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::attribute::Attribute;
use super::attribute_value::AttributeValue;
use super::rule::Rule;

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
#[diesel(belongs_to(AttributeValue))]
#[diesel(belongs_to(Rule))]
#[diesel(belongs_to(Attribute))]
#[diesel(table_name=rule_attribute_attributevalue)]
pub struct RuleAttributeAttributeValue {
    pub id: i32,
    pub attribute_value_id: i32,
    pub rule_id: i32,
    pub attribute_id: i32,
}

#[derive(Queryable, Deserialize, Insertable, ToSchema)]
#[diesel(table_name=rule_attribute_attributevalue)]
pub struct NewRuleAttributeAttributeValue {
    pub attribute_value_id: i32,
    pub rule_id: i32,
    pub attribute_id: i32,
}

#[derive(Queryable, Deserialize, ToSchema, Clone)]
pub struct NewRuleAttributeAttributeValueWithoutRule {
    pub attribute_value_id: i32,
    pub attribute_id: i32,
}
