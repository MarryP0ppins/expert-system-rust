use crate::{
    models::{
        answer::Answer,
        attribute_value::AttributeValue,
        clause::Clause,
        rule::{NewRule, Rule, RuleWithClausesAndEffects},
        rule_answer::RuleAnswer,
        rule_attributevalue::RuleAttributeValue,
    },
    schema::{answers, attributesvalues, rule_answer, rule_attributevalue, rules::dsl::*},
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_rules(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<RuleWithClausesAndEffects>, Error> {
    let _rules = rules
        .filter(system_id.eq(system))
        .load::<Rule>(connection)
        .await?;

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

pub async fn create_rule(
    connection: &mut AsyncPgConnection,
    rule_info: NewRule,
) -> Result<RuleWithClausesAndEffects, Error> {
    let _rule = insert_into(rules)
        .values::<NewRule>(rule_info)
        .get_result::<Rule>(connection)
        .await?;

    Ok(RuleWithClausesAndEffects {
        id: _rule.id,
        system_id: _rule.system_id,
        attribute_rule: _rule.attribute_rule,
        clauses: vec![],
        answers: match _rule.attribute_rule {
            true => None,
            false => Some(vec![]),
        },
        attributes_values: match _rule.attribute_rule {
            true => Some(vec![]),
            false => None,
        },
    })
}

pub async fn multiple_delete_rules(
    connection: &mut AsyncPgConnection,
    rules_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(rules.filter(id.eq_any(rules_ids)))
        .execute(connection)
        .await?)
}
