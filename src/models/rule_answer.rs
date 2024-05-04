use crate::schema::rule_answer;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::answer::Answer;
use super::question::Question;
use super::rule::Rule;

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Answer))]
#[diesel(belongs_to(Rule))]
#[diesel(belongs_to(Question))]
#[diesel(table_name=rule_answer)]
pub struct RuleAnswer {
    pub id: i32,
    pub answer_id: i32,
    pub rule_id: i32,
    pub question_id: i32,
}

#[derive(Queryable, Deserialize, Insertable, ToSchema)]
#[diesel(table_name=rule_answer)]
pub struct NewRuleAnswer {
    pub answer_id: i32,
    pub rule_id: i32,
    pub question_id: i32,
}
