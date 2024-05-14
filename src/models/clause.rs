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

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Identifiable,
    Associations,
    Selectable,
    Clone,
    ToSchema,
    Debug,
)]
#[diesel(belongs_to(Rule))]
#[diesel(table_name=clauses)]
pub struct Clause {
    pub id: i32,
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: String,
    pub operator: RuleOperator,
    pub question_id: i32,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=clauses)]
pub struct NewClause {
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: String,
    pub operator: RuleOperator,
    pub question_id: i32,
}

#[derive(Queryable, Deserialize, ToSchema)]
pub struct NewClauseWithoutRule {
    pub compared_value: String,
    pub logical_group: String,
    pub operator: RuleOperator,
    pub question_id: i32,
}

#[derive(Deserialize, AsChangeset, Clone, ToSchema)]
#[diesel(table_name=clauses)]
pub struct UpdateClause {
    pub id: i32,
    pub compared_value: Option<String>,
    pub logical_group: Option<String>,
    pub operator: Option<RuleOperator>,
    pub question_id: Option<i32>,
}
