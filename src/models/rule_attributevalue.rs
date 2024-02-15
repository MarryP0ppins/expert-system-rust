use crate::schema::rule_attributevalue;
use diesel::prelude::*;
use serde::Deserialize;

use super::attribute_value::AttributeValue;
use super::rule::Rule;

#[derive(Debug, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(AttributeValue))]
#[diesel(belongs_to(Rule))]
#[diesel(table_name=rule_attributevalue)]
pub struct RuleAttributevalue {
    pub id: i32,
    pub attribute_value_id: i32,
    pub rule_id: i32,
}

#[derive(Debug, Queryable, Deserialize, Insertable)]
#[diesel(table_name=rule_attributevalue)]
pub struct NewRuleAnswer {
    pub attribute_value_id: i32,
    pub rule_id: i32,
}
