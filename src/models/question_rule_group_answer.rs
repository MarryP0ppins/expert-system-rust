use crate::schema::questionrulegroup_answer;
use diesel::prelude::*;
use serde::Deserialize;

use super::answer::Answer;
use super::question_rule_group::QuestionRuleGroup;

#[derive(Debug, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Answer))]
#[diesel(belongs_to(QuestionRuleGroup))]
#[diesel(table_name=questionrulegroup_answer)]
pub struct QuestionRuleGroupAnswer {
    pub id: i32,
    pub answer_id: i32,
    pub question_rule_group_id: i32,
}

#[derive(Debug, Queryable, Deserialize, Insertable)]
#[diesel(table_name=questionrulegroup_answer)]
pub struct NewQuestionRuleGroupAnswer {
    pub answer_id: i32,
    pub question_rule_group_id: i32,
}
