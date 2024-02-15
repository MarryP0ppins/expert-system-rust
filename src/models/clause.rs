use super::rule::Rule;
use crate::schema::{clauses, sql_types::Operatorenum};
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

#[derive(
    Debug, Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable, Clone,
)]
#[diesel(belongs_to(Rule))]
#[diesel(table_name=clauses)]
pub struct Clause {
    pub id: i32,
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: i32,
    pub operator: RuleOperator,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=clauses)]
pub struct NewClause {
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: i32,
    pub operator: RuleOperator,
}

#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name=clauses)]
pub struct UpdateClause {
    pub id: i32,
    pub compared_value: Option<String>,
    pub logical_group: Option<i32>,
    pub operator: Option<RuleOperator>,
}
