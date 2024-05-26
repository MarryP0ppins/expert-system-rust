pub struct RuleAttributeAttributeValue {
    pub id: i32,
    pub attribute_value_id: i32,
    pub rule_id: i32,
    pub attribute_id: i32,
}

pub struct NewRuleAttributeAttributeValue {
    pub attribute_value_id: i32,
    pub rule_id: i32,
    pub attribute_id: i32,
}

pub struct NewRuleAttributeAttributeValueWithoutRule {
    pub attribute_value_id: i32,
    pub attribute_id: i32,
}
