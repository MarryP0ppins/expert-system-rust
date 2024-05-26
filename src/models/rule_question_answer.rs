pub struct RuleQuestionAnswer {
    pub id: i32,
    pub answer_id: i32,
    pub rule_id: i32,
    pub question_id: i32,
}

pub struct NewRuleQuestionAnswer {
    pub answer_id: i32,
    pub rule_id: i32,
    pub question_id: i32,
}

pub struct NewRuleQuestionAnswerWithoutRule {
    pub answer_id: i32,
    pub question_id: i32,
}
