use crate::{
    models::{
        answer::Answer,
        attribute_value::AttributeValue,
        clause::{Clause, NewClause},
        rule::{NewRule, NewRuleWithClausesAndEffects, Rule, RuleWithClausesAndEffects},
        rule_answer::{NewRuleAnswer, RuleAnswer},
        rule_attributevalue::{NewRuleAttributeValue, RuleAttributeValue},
    },
    schema::{answers, attributesvalues, clauses, rule_answer, rule_attributevalue, rules::dsl::*},
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};

pub async fn get_rules(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<RuleWithClausesAndEffects>, Error> {
    let _rules: Vec<Rule>;
    match rules
        .filter(system_id.eq(system))
        .load::<Rule>(connection)
        .await
    {
        Ok(ok) => _rules = ok,
        Err(err) => return Err(err),
    };

    let _grouped_answers: Vec<Vec<(RuleAnswer, Answer)>>;
    match RuleAnswer::belonging_to(&_rules)
        .inner_join(answers::table)
        .select((rule_answer::all_columns, answers::all_columns))
        .load::<(RuleAnswer, Answer)>(connection)
        .await
    {
        Ok(ok) => _grouped_answers = ok.grouped_by(&_rules),
        Err(_) => _grouped_answers = vec![],
    };

    let _grouped_attributesvalues: Vec<Vec<(RuleAttributeValue, AttributeValue)>>;
    match RuleAttributeValue::belonging_to(&_rules)
        .inner_join(attributesvalues::table)
        .select((
            rule_attributevalue::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(RuleAttributeValue, AttributeValue)>(connection)
        .await
    {
        Ok(ok) => _grouped_attributesvalues = ok.grouped_by(&_rules),
        Err(_) => _grouped_attributesvalues = vec![],
    };

    let _grouped_clauses: Vec<Vec<Clause>>;
    match Clause::belonging_to(&_rules)
        .load::<Clause>(connection)
        .await
    {
        Ok(ok) => _grouped_clauses = ok.grouped_by(&_rules),
        Err(_) => _grouped_clauses = vec![],
    };

    let result = _rules
        .into_iter()
        .zip(_grouped_answers)
        .zip(_grouped_attributesvalues)
        .zip(_grouped_clauses)
        .map(
            |(((_rule, _answers), _attributesvalues), _clauses)| RuleWithClausesAndEffects {
                id: _rule.id,
                system_id: _rule.system_id,
                attribute_rule: _rule.attribute_rule,
                answers: match _rule.attribute_rule {
                    false => Some(_answers.into_iter().map(|(_, answer)| answer).collect()),
                    true => None,
                },
                clauses: _clauses,
                attributes_values: match _rule.attribute_rule {
                    true => Some(
                        _attributesvalues
                            .into_iter()
                            .map(|(_, attributevalue)| attributevalue)
                            .collect(),
                    ),
                    false => None,
                },
            },
        )
        .collect::<Vec<RuleWithClausesAndEffects>>();

    Ok(result)
}

pub async fn create_rules(
    connection: &mut AsyncPgConnection,
    rule_info: Vec<NewRuleWithClausesAndEffects>,
) -> Result<Vec<RuleWithClausesAndEffects>, Error> {
    let (new_rules, new_clauses, answers_ids, attribues_values_ids) =
        rule_info
            .into_iter()
            .fold((vec![], vec![], vec![], vec![]), |mut acc, raw| {
                acc.0.push(NewRule {
                    system_id: raw.system_id,
                    attribute_rule: raw.attribute_rule,
                });
                acc.1.push(raw.clauses);
                acc.2.push(raw.answers);
                acc.3.push(raw.attributes_values);
                acc
            });

    let mut _rule: Vec<Rule> = vec![];
    let mut _grouped_clauses: Vec<Vec<Clause>> = vec![];

    match connection
        .transaction(|connection| {
            async {
                match insert_into(rules)
                    .values::<Vec<NewRule>>(new_rules)
                    .get_results::<Rule>(connection)
                    .await
                {
                    Ok(ok) => _rule = ok,
                    Err(err) => return Err(err),
                };

                match insert_into(clauses::table)
                    .values::<Vec<NewClause>>(
                        new_clauses
                            .into_iter()
                            .zip(&_rule)
                            .flat_map(|(clauses, rule)| {
                                clauses.into_iter().map(|clause| NewClause {
                                    rule_id: rule.id,
                                    compared_value: clause.compared_value,
                                    logical_group: clause.logical_group,
                                    operator: clause.operator,
                                })
                            })
                            .collect(),
                    )
                    .get_results::<Clause>(connection)
                    .await
                {
                    Ok(ok) => _grouped_clauses = ok.grouped_by(&_rule),
                    Err(err) => return Err(err),
                };

                let (_rules_answers, _rules_attributes_values) = _rule
                    .iter()
                    .zip(answers_ids)
                    .zip(attribues_values_ids)
                    .fold(
                        (vec![], vec![]),
                        |mut acc, ((rule, answers), attributes_values)| {
                            match rule.attribute_rule {
                                true => attributes_values.into_iter().for_each(|value_id| {
                                    acc.1.push(NewRuleAttributeValue {
                                        attribute_value_id: value_id,
                                        rule_id: rule.id,
                                    })
                                }),
                                false => answers.into_iter().for_each(|value_id| {
                                    acc.0.push(NewRuleAnswer {
                                        answer_id: value_id,
                                        rule_id: rule.id,
                                    })
                                }),
                            }
                            acc
                        },
                    );

                match insert_into(rule_answer::table)
                    .values::<Vec<NewRuleAnswer>>(_rules_answers)
                    .execute(connection)
                    .await
                {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                };
                match insert_into(rule_attributevalue::table)
                    .values::<Vec<NewRuleAttributeValue>>(_rules_attributes_values)
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

    let _grouped_answers: Vec<Vec<(RuleAnswer, Answer)>>;
    match RuleAnswer::belonging_to(&_rule)
        .inner_join(answers::table)
        .select((rule_answer::all_columns, answers::all_columns))
        .load::<(RuleAnswer, Answer)>(connection)
        .await
    {
        Ok(ok) => _grouped_answers = ok.grouped_by(&_rule),
        Err(_) => _grouped_answers = vec![],
    };

    let _grouped_attributes_values: Vec<Vec<(RuleAttributeValue, AttributeValue)>>;
    match RuleAttributeValue::belonging_to(&_rule)
        .inner_join(attributesvalues::table)
        .select((
            rule_attributevalue::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(RuleAttributeValue, AttributeValue)>(connection)
        .await
    {
        Ok(ok) => _grouped_attributes_values = ok.grouped_by(&_rule),
        Err(_) => _grouped_attributes_values = vec![],
    };

    let result = _rule
        .into_iter()
        .zip(_grouped_clauses)
        .zip(_grouped_answers)
        .zip(_grouped_attributes_values)
        .map(
            |(((rule, clauses), answers), attribute_vlues)| RuleWithClausesAndEffects {
                id: rule.id,
                system_id: rule.system_id,
                attribute_rule: rule.attribute_rule,
                clauses,
                answers: match rule.attribute_rule {
                    false => Some(answers.into_iter().map(|(_, answer)| answer).collect()),
                    true => None,
                },
                attributes_values: match rule.attribute_rule {
                    true => Some(
                        attribute_vlues
                            .into_iter()
                            .map(|(_, attributevalue)| attributevalue)
                            .collect(),
                    ),
                    false => None,
                },
            },
        )
        .collect();

    Ok(result)
}

pub async fn multiple_delete_rules(
    connection: &mut AsyncPgConnection,
    rules_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(rules.filter(id.eq_any(rules_ids)))
        .execute(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
