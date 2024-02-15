use super::{answer::Answer, attribute_value::AttributeValue, clause::Clause, system::System};
use crate::schema::rules;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
/*
* User models begin from here
*/

#[derive(Debug, Queryable, Serialize, Identifiable, Associations, Selectable, Clone)]
#[diesel(belongs_to(System))]
#[diesel(table_name=rules)]
pub struct Rule {
    pub id: i32,
    pub system_id: i32,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=rules)]
pub struct NewRule {
    pub system_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct RuleWithClausesAndEffects {
    pub id: i32,
    pub system_id: i32,
    pub clauses: Vec<Clause>,
    pub answers: Option<Vec<Answer>>,
    pub attributes_values: Option<Vec<AttributeValue>>,
}
