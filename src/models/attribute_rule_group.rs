use crate::schema::attributerulegroups;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::{attribute_value::AttributeValue, rules::{NewRule, Rule}};

#[derive(Debug, Queryable, Serialize, Identifiable, Selectable)]
#[diesel(table_name=attributerulegroups)]
pub struct AttributeRuleGroup {
    pub id: i32,
    pub system_id: i32,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=attributerulegroups)]
pub struct NewAttributeRuleGroup {
    pub system_id: i32,
}

#[derive(Debug, Deserialize, Queryable)]
pub struct NewAttributeRuleGroupWithRulesAndAttributesValues {
    pub system_id: i32,
    pub rules: Vec<NewRule>,
    pub attributes_values_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct AttributeRuleGroupWithRulesAndAttributesValues {
    pub id: i32,
    pub system_id: i32,
    pub rules: Vec<Rule>,
    pub attributes_values: Vec<AttributeValue>,
}
