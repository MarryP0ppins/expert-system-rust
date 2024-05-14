use crate::{
    models::{
        clause::{Clause, NewClause},
        rule::{NewRule, NewRuleWithClausesAndEffects, Rule, RuleWithClausesAndEffects},
        rule_attribute_attributevalue::{
            NewRuleAttributeAttributeValue, RuleAttributeAttributeValue,
        },
        rule_question_answer::{NewRuleQuestionAnswer, RuleQuestionAnswer},
    },
    schema::{clauses, rule_attribute_attributevalue, rule_question_answer, rules::dsl::*},
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};

pub async fn get_rules(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<RuleWithClausesAndEffects>, Error> {
    let _rules = rules
        .filter(system_id.eq(system))
        .load::<Rule>(connection)
        .await?;

    let _grouped_answers: Vec<Vec<RuleQuestionAnswer>>;
    match RuleQuestionAnswer::belonging_to(&_rules)
        .load::<RuleQuestionAnswer>(connection)
        .await
    {
        Ok(ok) => _grouped_answers = ok.grouped_by(&_rules),
        Err(_) => _grouped_answers = vec![],
    };

    let _grouped_attributesvalues: Vec<Vec<RuleAttributeAttributeValue>>;
    match RuleAttributeAttributeValue::belonging_to(&_rules)
        .load::<RuleAttributeAttributeValue>(connection)
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
                clauses: _clauses,
                rule_question_answer_ids: _answers,
                rule_attribute_attributevalue_ids: _attributesvalues,
            },
        )
        .collect::<Vec<RuleWithClausesAndEffects>>();

    Ok(result)
}

pub async fn create_rule(
    connection: &mut AsyncPgConnection,
    rule_info: Vec<NewRuleWithClausesAndEffects>,
) -> Result<Vec<RuleWithClausesAndEffects>, Error> {
    let (_rules, _clauses, _rule_question_answer_ids, _rule_attribute_attributevalue_ids) =
        rule_info
            .into_iter()
            .fold((vec![], vec![], vec![], vec![]), |mut acc, raw| {
                acc.0.push(NewRule {
                    system_id: raw.system_id,
                    attribute_rule: raw.attribute_rule,
                });
                acc.1.push(raw.clauses);
                acc.2.push(raw.rule_question_answer_ids);
                acc.3.push(raw.rule_attribute_attributevalue_ids);
                acc
            });

    let mut new_rules: Vec<Rule> = vec![];
    let mut new_clauses: Vec<Vec<Clause>> = vec![];
    let mut new_rule_question_answers: Vec<Vec<RuleQuestionAnswer>> = vec![];
    let mut new_rule_attribute_atributevalues: Vec<Vec<RuleAttributeAttributeValue>> = vec![];

    match connection
        .transaction(|connection| {
            async {
                new_rules = insert_into(rules)
                    .values::<Vec<NewRule>>(_rules)
                    .get_results::<Rule>(connection)
                    .await?;

                new_clauses = insert_into(clauses::table)
                    .values::<Vec<NewClause>>(
                        _clauses
                            .into_iter()
                            .zip(&new_rules)
                            .flat_map(|(clauses, rule)| {
                                clauses.into_iter().map(|value| NewClause {
                                    question_id: value.question_id,
                                    rule_id: rule.id,
                                    compared_value: value.compared_value,
                                    logical_group: value.logical_group,
                                    operator: value.operator,
                                })
                            })
                            .collect(),
                    )
                    .get_results::<Clause>(connection)
                    .await?
                    .grouped_by(&new_rules);

                new_rule_question_answers = insert_into(rule_question_answer::table)
                    .values::<Vec<NewRuleQuestionAnswer>>(
                        _rule_question_answer_ids
                            .into_iter()
                            .zip(&new_rules)
                            .flat_map(|(rule_question_answer_ids, rule)| {
                                rule_question_answer_ids.into_iter().map(
                                    |rule_question_answer_id| NewRuleQuestionAnswer {
                                        answer_id: rule_question_answer_id.answer_id,
                                        question_id: rule_question_answer_id.question_id,
                                        rule_id: rule.id,
                                    },
                                )
                            })
                            .collect(),
                    )
                    .get_results::<RuleQuestionAnswer>(connection)
                    .await?
                    .grouped_by(&new_rules);

                new_rule_attribute_atributevalues =
                    insert_into(rule_attribute_attributevalue::table)
                        .values::<Vec<NewRuleAttributeAttributeValue>>(
                            _rule_attribute_attributevalue_ids
                                .into_iter()
                                .zip(&new_rules)
                                .flat_map(|(rule_attribute_atributevalue_ids, rule)| {
                                    rule_attribute_atributevalue_ids.into_iter().map(
                                        |rule_attribute_atributevalue_id| {
                                            NewRuleAttributeAttributeValue {
                                                attribute_id: rule_attribute_atributevalue_id
                                                    .attribute_id,
                                                attribute_value_id: rule_attribute_atributevalue_id
                                                    .attribute_value_id,
                                                rule_id: rule.id,
                                            }
                                        },
                                    )
                                })
                                .collect(),
                        )
                        .get_results::<RuleAttributeAttributeValue>(connection)
                        .await?
                        .grouped_by(&new_rules);
                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    let result = new_rules
        .into_iter()
        .zip(new_clauses)
        .zip(new_rule_question_answers)
        .zip(new_rule_attribute_atributevalues)
        .map(
            |(((rule, clause), rule_question_answer), rule_attribute_atributevalue)| {
                RuleWithClausesAndEffects {
                    id: rule.id,
                    system_id: rule.system_id,
                    attribute_rule: rule.attribute_rule,
                    clauses: clause,
                    rule_question_answer_ids: rule_question_answer,
                    rule_attribute_attributevalue_ids: rule_attribute_atributevalue,
                }
            },
        )
        .collect::<Vec<RuleWithClausesAndEffects>>();

    Ok(result)
}

pub async fn multiple_delete_rules(
    connection: &mut AsyncPgConnection,
    rules_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(rules.filter(id.eq_any(rules_ids)))
        .execute(connection)
        .await?)
}
