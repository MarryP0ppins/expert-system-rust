use crate::{
    models::{
        attribute_rule_group::{
            AttributeRuleGroup, AttributeRuleGroupWithRulesAndAttributesValues,
            NewAttributeRuleGroup, NewAttributeRuleGroupWithRulesAndAttributesValues,
        },
        attribute_rule_group_attribute_value::{
            AttributeRuleGroupAttributeValue, NewAttributeRuleGroupAttributeValue,
        },
        attribute_value::AttributeValue,
        rules::{NewRule, Rule},
    },
    schema::{
        attributerulegroup_atributevalue, attributerulegroups::dsl::*, attributesvalues, rules,
    },
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};

pub async fn get_attribute_rule_groups(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<AttributeRuleGroupWithRulesAndAttributesValues>, Error> {
    let _attribute_rule_group: Vec<AttributeRuleGroup>;
    match attributerulegroups
        .filter(system_id.eq(system))
        .load::<AttributeRuleGroup>(connection)
        .await
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
        .await
    {
        Ok(ok) => _grouped_attributes_values = ok.grouped_by(&_attribute_rule_group),
        Err(_) => _grouped_attributes_values = vec![],
    };

    let _grouped_rules: Vec<Vec<Rule>>;
    match Rule::belonging_to(&_attribute_rule_group)
        .load::<Rule>(connection)
        .await
    {
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

pub async fn create_attribute_rule_groups(
    connection: &mut AsyncPgConnection,
    attribute_rule_group_info: Vec<NewAttributeRuleGroupWithRulesAndAttributesValues>,
) -> Result<Vec<AttributeRuleGroupWithRulesAndAttributesValues>, Error> {
    let (new_rules, attributes_values_ids, attribute_rule_group_system_ids) =
        attribute_rule_group_info
            .into_iter()
            .fold((vec![], vec![], vec![]), |mut acc, raw| {
                acc.0.push(raw.rules);
                acc.1.push(raw.attributes_values_ids);
                acc.2.push(NewAttributeRuleGroup {
                    system_id: raw.system_id,
                });
                acc
            });

    let mut _attribute_rule_group: Vec<AttributeRuleGroup> = vec![];
    let mut _grouped_rules: Vec<Vec<Rule>> = vec![];

    match connection
        .transaction(|connection| {
            async {
                match insert_into(attributerulegroups)
                    .values::<Vec<NewAttributeRuleGroup>>(attribute_rule_group_system_ids)
                    .get_results::<AttributeRuleGroup>(connection)
                    .await
                {
                    Ok(ok) => _attribute_rule_group = ok,
                    Err(err) => return Err(err),
                };

                match insert_into(rules::table)
                    .values::<Vec<NewRule>>(
                        new_rules
                            .into_iter()
                            .zip(&_attribute_rule_group)
                            .flat_map(|(rules, group)| {
                                rules.into_iter().map(|rule| NewRule {
                                    attribute_rule_group_id: Some(group.id),
                                    question_rule_group_id: None,
                                    compared_value: rule.compared_value.clone(),
                                    logical_group: rule.logical_group,
                                    operator: rule.operator.clone(),
                                })
                            })
                            .collect(),
                    )
                    .get_results::<Rule>(connection)
                    .await
                {
                    Ok(ok) => _grouped_rules = ok.grouped_by(&_attribute_rule_group),
                    Err(err) => return Err(err),
                };

                match insert_into(attributerulegroup_atributevalue::table)
                    .values::<Vec<NewAttributeRuleGroupAttributeValue>>(
                        attributes_values_ids
                            .into_iter()
                            .zip(&_attribute_rule_group)
                            .flat_map(|(attributes_values, group)| {
                                attributes_values.into_iter().map(|value| {
                                    NewAttributeRuleGroupAttributeValue {
                                        attribute_value_id: value,
                                        attribute_rule_group_id: group.id,
                                    }
                                })
                            })
                            .collect(),
                    )
                    .execute(connection)
                    .await
                {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                };

                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => (),
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
        .await
    {
        Ok(ok) => _grouped_attributes_values = ok.grouped_by(&_attribute_rule_group),
        Err(_) => _grouped_attributes_values = vec![],
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

pub async fn multiple_delete_attribute_rule_groups(
    connection: &mut AsyncPgConnection,
    attribute_rule_groups_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(attributerulegroups.filter(id.eq_any(attribute_rule_groups_ids)))
        .execute(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
