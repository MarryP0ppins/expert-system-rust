use super::{
    clause::{Clause, NewClauseWithoutRule},
    rule_attribute_attributevalue::{
        NewRuleAttributeAttributeValueWithoutRule, RuleAttributeAttributeValue,
    },
    rule_question_answer::{NewRuleQuestionAnswerWithoutRule, RuleQuestionAnswer},
    system::System,
};
use crate::schema::rules;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable, Clone, Debug,
)]
#[diesel(belongs_to(System))]
#[diesel(table_name=rules)]
pub struct Rule {
    pub id: i32,
    pub system_id: i32,
    pub attribute_rule: bool,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=rules)]
pub struct NewRule {
    pub system_id: i32,
    pub attribute_rule: bool,
}

#[derive(Queryable, Deserialize, ToSchema)]
pub struct NewRuleWithClausesAndEffects {
    pub system_id: i32,
    pub attribute_rule: bool,
    pub clauses: Vec<NewClauseWithoutRule>,
    pub rule_question_answer_ids: Vec<NewRuleQuestionAnswerWithoutRule>,
    pub rule_attribute_attributevalue_ids: Vec<NewRuleAttributeAttributeValueWithoutRule>,
}

#[derive(Queryable, Serialize, ToSchema, Clone)]
pub struct RuleWithClausesAndEffects {
    pub id: i32,
    pub system_id: i32,
    pub attribute_rule: bool,
    pub clauses: Vec<Clause>,
    pub rule_question_answer_ids: Vec<RuleQuestionAnswer>,
    pub rule_attribute_attributevalue_ids: Vec<RuleAttributeAttributeValue>,
}
