use super::{answer::Answer, attribute_value::AttributeValue, clause::Clause, system::System};
use crate::schema::rules;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Serialize, Identifiable, Associations, Selectable, Clone)]
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

#[derive(Queryable, Serialize, ToSchema)]
pub struct RuleWithClausesAndEffects {
    pub id: i32,
    pub system_id: i32,
    pub attribute_rule: bool,
    pub clauses: Vec<Clause>,
    pub answers: Option<Vec<Answer>>,
    pub attributes_values: Option<Vec<AttributeValue>>,
}
