use crate::schema::questionrulegroups;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    answer::Answer,
    rules::{NewRule, Rule},
};

#[derive(Debug, Queryable, Serialize, Identifiable, Selectable)]
#[diesel(table_name=questionrulegroups)]
pub struct QuestionRuleGroup {
    pub id: i32,
    pub system_id: i32,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=questionrulegroups)]
pub struct NewQuestionRuleGroup {
    pub system_id: i32,
}

#[derive(Debug, Deserialize, Queryable)]
pub struct NewQuestionRuleGroupWithRulesAndAnswers {
    pub system_id: i32,
    pub rules: Vec<NewRule>,
    pub answers: Vec<i32>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct QuestionRuleGroupWithRulesAndAnswers {
    pub id: i32,
    pub system_id: i32,
    pub rules: Vec<Rule>,
    pub answers: Vec<Answer>,
}
