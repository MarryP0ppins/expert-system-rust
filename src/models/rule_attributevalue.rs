use crate::schema::rule_attributevalue;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::attribute::Attribute;
use super::attribute_value::AttributeValue;
use super::rule::Rule;

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(AttributeValue))]
#[diesel(belongs_to(Rule))]
#[diesel(belongs_to(Attribute))]
#[diesel(table_name=rule_attributevalue)]
pub struct RuleAttributeValue {
    pub id: i32,
    pub attribute_value_id: i32,
    pub rule_id: i32,
    pub attribute_id: i32,
}

#[derive(Queryable, Deserialize, Insertable, ToSchema)]
#[diesel(table_name=rule_attributevalue)]
pub struct NewRuleAttributeValue {
    pub attribute_value_id: i32,
    pub rule_id: i32,
    pub attribute_id: i32,
}
