use crate::{
    models::{
        attribute_rule_group::{
            AttributeRuleGroup, AttributeRuleGroupWithRulesAndAttributesValues,
            NewAttributeRuleGroup, NewAttributeRuleGroupWithRulesAndAttributesValues,
        },
        attribute_rule_group_attribute_value::AttributeRuleGroupAttributeValue,
        attribute_value::AttributeValue,
        rules::{NewRule, Rule},
    },
    schema::{
        attributerulegroup_atributevalue, attributerulegroups::dsl::*, attributesvalues, rules,
    },
};
use diesel::{delete, insert_into, prelude::*, result::Error};

pub fn get_attribute_rule_groups(
    connection: &mut PgConnection,
    system: i32,
) -> Result<Vec<AttributeRuleGroupWithRulesAndAttributesValues>, Error> {
    let _attribute_rule_group: Vec<AttributeRuleGroup>;
    match attributerulegroups
        .filter(system_id.eq(system))
        .load::<AttributeRuleGroup>(connection)
    {
        Ok(ok) => _attribute_rule_group = ok,
        Err(err) => return Err(err),
    };

    let _grouped_attributes_values: Vec<Vec<(AttributeRuleGroupAttributeValue, AttributeValue)>>;
    match AttributeRuleGroupAttributeValue::belonging_to(&_attribute_rule_group)
        .inner_join(attributesvalues::table)
        .select((
            attributerulegroup_atributevalue::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(AttributeRuleGroupAttributeValue, AttributeValue)>(connection)
    {
        Ok(ok) => _grouped_attributes_values = ok.grouped_by(&_attribute_rule_group),
        Err(_) => _grouped_attributes_values = vec![],
    };

    let _grouped_rules: Vec<Vec<Rule>>;
    match Rule::belonging_to(&_attribute_rule_group).load::<Rule>(connection) {
        Ok(ok) => _grouped_rules = ok.grouped_by(&_attribute_rule_group),
        Err(_) => _grouped_rules = vec![],
    };

    let result = _attribute_rule_group
        .into_iter()
        .zip(_grouped_attributes_values)
        .zip(_grouped_rules)
        .map(|((_attribute_rule_group, _attributes_values), _rules)| {
            AttributeRuleGroupWithRulesAndAttributesValues {
                id: _attribute_rule_group.id,
                system_id: _attribute_rule_group.system_id,
                rules: _rules,
                attributes_values: _attributes_values
                    .into_iter()
                    .map(|(_, attribute_values)| attribute_values)
                    .collect(),
            }
        })
        .collect::<Vec<AttributeRuleGroupWithRulesAndAttributesValues>>();

    Ok(result)
}

pub fn create_attribute_rule_group(
    connection: &mut PgConnection,
    attribute_rule_group_info: NewAttributeRuleGroupWithRulesAndAttributesValues,
) -> Result<AttributeRuleGroupWithRulesAndAttributesValues, Error> {
    let new_rules = attribute_rule_group_info.rules;
    let attributes_values_ids = attribute_rule_group_info.attributes_values_ids;

    let _attribute_rule_group: AttributeRuleGroup;
    match insert_into(attributerulegroups)
        .values::<NewAttributeRuleGroup>(NewAttributeRuleGroup {
            system_id: attribute_rule_group_info.system_id,
        })
        .get_result::<AttributeRuleGroup>(connection)
    {
        Ok(ok) => _attribute_rule_group = ok,
        Err(err) => return Err(err),
    };

    let _attribute_rule_group_rules: Vec<Rule>;
    match insert_into(rules::table)
        .values::<Vec<NewRule>>(
            new_rules
                .iter()
                .map(|rule_body| NewRule {
                    attribute_rule_group_id: Some(_attribute_rule_group.id),
                    question_rule_group_id: None,
                    compared_value: rule_body.compared_value.clone(),
                    logical_group: rule_body.logical_group,
                    operator: rule_body.operator.clone(),
                })
                .collect(),
        )
        .get_results::<Rule>(connection)
    {
        Ok(ok) => _attribute_rule_group_rules = ok,
        Err(err) => return Err(err),
    };

    let _attributes_values: Vec<AttributeValue>;
    match attributesvalues::table
        .filter(attributesvalues::id.eq_any(attributes_values_ids))
        .load::<AttributeValue>(connection)
    {
        Ok(ok) => _attributes_values = ok,
        Err(err) => return Err(err),
    };

    let result = AttributeRuleGroupWithRulesAndAttributesValues {
        id: _attribute_rule_group.id,
        system_id: _attribute_rule_group.system_id,
        rules: _attribute_rule_group_rules,
        attributes_values: _attributes_values,
    };

    Ok(result)
}

pub fn multiple_delete_attribute_rule_groups(
    connection: &mut PgConnection,
    attribute_rule_groups_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(attributerulegroups.filter(id.eq_any(attribute_rule_groups_ids)))
        .execute(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
