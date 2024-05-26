use super::{
    clause::{Clause, NewClauseWithoutRule},
    rule_attribute_attributevalue::{
        NewRuleAttributeAttributeValueWithoutRule, RuleAttributeAttributeValue,
    },
    rule_question_answer::{NewRuleQuestionAnswerWithoutRule, RuleQuestionAnswer},
};

pub struct Rule {
    pub id: i32,
    pub system_id: i32,
    pub attribute_rule: bool,
}

pub struct NewRule {
    pub system_id: i32,
    pub attribute_rule: bool,
}

pub struct NewRuleWithClausesAndEffects {
    pub system_id: i32,
    pub attribute_rule: bool,
    pub clauses: Vec<NewClauseWithoutRule>,
    pub rule_question_answer_ids: Vec<NewRuleQuestionAnswerWithoutRule>,
    pub rule_attribute_attributevalue_ids: Vec<NewRuleAttributeAttributeValueWithoutRule>,
}

pub struct RuleWithClausesAndEffects {
    pub id: i32,
    pub system_id: i32,
    pub attribute_rule: bool,
    pub clauses: Vec<Clause>,
    pub rule_question_answer_ids: Vec<RuleQuestionAnswer>,
    pub rule_attribute_attributevalue_ids: Vec<RuleAttributeAttributeValue>,
}
