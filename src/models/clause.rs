use super::rule::Rule;
use crate::schema::{clauses, sql_types::Operatorenum};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, DbEnum, Deserialize, Serialize, Clone, ToSchema)]
#[ExistingTypePath = "Operatorenum"]
pub enum RuleOperator {
    Equel,
    NotEqual,
    Below,
    Above,
    NoMoreThan,
    NoLessThan,
}

#[derive(Debug, Queryable, Serialize, Identifiable, Associations, Selectable, Clone, ToSchema)]
#[diesel(belongs_to(Rule))]
#[diesel(table_name=clauses)]
pub struct Clause {
    pub id: i32,
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: i32,
    pub operator: RuleOperator,
    pub question_id: i32,
}

#[derive(Debug, Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=clauses)]
pub struct NewClause {
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: i32,
    pub operator: RuleOperator,
    pub question_id: i32,
}

#[derive(Debug, Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=clauses)]
pub struct UpdateClause {
    pub id: i32,
    pub compared_value: Option<String>,
    pub logical_group: Option<i32>,
    pub operator: Option<RuleOperator>,
    pub question_id: Option<i32>,
}
