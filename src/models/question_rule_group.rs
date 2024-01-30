use crate::schema::questionrulegroups;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::answer::NewAnswer;

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
    pub rules: i32,
    pub answers: Vec<NewAnswer>,
}
