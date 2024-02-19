use crate::schema::rule_answer;
use diesel::prelude::*;
use serde::Deserialize;
use utoipa::ToSchema;

use super::answer::Answer;
use super::rule::Rule;

#[derive(Debug, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Answer))]
#[diesel(belongs_to(Rule))]
#[diesel(table_name=rule_answer)]
pub struct RuleAnswer {
    pub id: i32,
    pub answer_id: i32,
    pub rule_id: i32,
}

#[derive(Debug, Queryable, Deserialize, Insertable, ToSchema)]
#[diesel(table_name=rule_answer)]
pub struct NewRuleAnswer {
    pub answer_id: i32,
    pub rule_id: i32,
}
