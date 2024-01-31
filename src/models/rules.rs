use super::attribute_rule_group::AttributeRuleGroup;
use super::question_rule_group::QuestionRuleGroup;
use crate::schema::{rules, sql_types::Operatorenum};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
/*
* User models begin from here
*/

#[derive(Debug, DbEnum, Deserialize, Serialize, Clone)]
#[ExistingTypePath = "Operatorenum"]

pub enum RuleOperator {
    Equel,
    NotEqual,
    Below,
    Above,
    NoMoreThan,
    NoLessThan,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(QuestionRuleGroup))]
#[diesel(belongs_to(AttributeRuleGroup))]
#[diesel(table_name=rules)]
pub struct Rule {
    pub id: i32,
    pub attribute_rule_group_id: Option<i32>,
    pub question_rule_group_id: Option<i32>,
    pub compared_value: String,
    pub logical_group: i32,
    pub operator: RuleOperator,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=rules)]
pub struct NewRule {
    pub attribute_rule_group_id: Option<i32>,
    pub question_rule_group_id: Option<i32>,
    pub compared_value: String,
    pub logical_group: i32,
    pub operator: RuleOperator,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name=rules)]
pub struct UpdateRule {
    pub compared_value: Option<String>,
    pub logical_group: Option<i32>,
    pub operator: Option<RuleOperator>,
}
