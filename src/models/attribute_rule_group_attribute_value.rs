use crate::schema::attributerulegroup_atributevalue;
use diesel::prelude::*;
use serde::Deserialize;

use super::attribute_rule_group::AttributeRuleGroup;
use super::attribute_value::AttributeValue;

#[derive(Debug, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(AttributeValue))]
#[diesel(belongs_to(AttributeRuleGroup))]
#[diesel(table_name=attributerulegroup_atributevalue)]
pub struct AttributeRuleGroupAttributeValue {
    pub id: i32,
    pub attribute_value_id: i32,
    pub attribute_rule_group_id: i32,
}

#[derive(Debug, Queryable, Deserialize, Insertable)]
#[diesel(table_name=attributerulegroup_atributevalue)]
pub struct NewAttributeRuleGroupAttributeValue {
    pub attribute_value_id: i32,
    pub attribute_rule_group_id: i32,
}
