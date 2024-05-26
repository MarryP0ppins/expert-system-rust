pub enum RuleOperator {
    Equal,
    NotEqual,
    Below,
    Above,
    NoMoreThan,
    NoLessThan,
}

pub struct Clause {
    pub id: i32,
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: String,
    pub operator: RuleOperator,
    pub question_id: i32,
}

pub struct NewClause {
    pub rule_id: i32,
    pub compared_value: String,
    pub logical_group: String,
    pub operator: RuleOperator,
    pub question_id: i32,
}

pub struct NewClauseWithoutRule {
    pub compared_value: String,
    pub logical_group: String,
    pub operator: RuleOperator,
    pub question_id: i32,
}

pub struct UpdateClause {
    pub id: i32,
    pub compared_value: Option<String>,
    pub logical_group: Option<String>,
    pub operator: Option<RuleOperator>,
    pub question_id: Option<i32>,
}
