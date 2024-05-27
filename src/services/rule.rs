use std::sync::Arc;

use crate::{
    entity::{
        clauses::{Entity as ClauseEntity, Model as ClauseModel},
        rule_attribute_attributevalue::{
            Entity as RuleAttributeAttributeValueEntity, Model as RuleAttributeAttributeValueModel,
        },
        rule_question_answer::{
            Entity as RuleQuestionAnswerEntity, Model as RuleQuestionAnswerModel,
        },
        rules::{
            ActiveModel as RuleActiveModel, Column as RuleColumn, Entity as RuleEntity,
            NewRuleWithClausesAndEffects, RuleWithClausesAndEffects,
        },
    },
    services::rule_attribute_attributevalue::create_rule_attribute_attributevalues,
};
use futures::future::try_join_all;
use sea_orm::*;
use tokio::try_join;

use super::{clause::create_clauses, rule_question_answer::create_rule_question_answers};

pub async fn get_rules<C>(db: &C, system_id: i32) -> Result<Vec<RuleWithClausesAndEffects>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut rules = RuleEntity::find()
        .filter(RuleColumn::SystemId.eq(system_id))
        .all(db)
        .await?;
    rules.sort_by_key(|rule| rule.id);

    let (grouped_answers, grouped_attributesvalues, grouped_clauses) = try_join!(
        rules.load_many(RuleQuestionAnswerEntity, db),
        rules.load_many(RuleAttributeAttributeValueEntity, db),
        rules.load_many(ClauseEntity, db),
    )?;

    let result = rules
        .into_iter()
        .zip(grouped_answers)
        .zip(grouped_attributesvalues)
        .zip(grouped_clauses)
        .map(
            |(((_rule, mut _answers), mut _attributesvalues), mut _clauses)| {
                _answers.sort_by_key(|answer| answer.id);
                _attributesvalues.sort_by_key(|values| values.id);
                _clauses.sort_by_key(|clause| clause.id);
                RuleWithClausesAndEffects {
                    id: _rule.id,
                    system_id: _rule.system_id,
                    attribute_rule: _rule.attribute_rule,
                    clauses: _clauses,
                    rule_question_answer_ids: _answers,
                    rule_attribute_attributevalue_ids: _attributesvalues,
                }
            },
        )
        .collect();

    Ok(result)
}

pub async fn create_rule<C>(
    db: &C,
    rule_info: Vec<NewRuleWithClausesAndEffects>,
) -> Result<Vec<RuleWithClausesAndEffects>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = db.begin().await?;
    let shared_txn = Arc::new(&txn);

    let new_rules = rule_info.into_iter().map(|rule_raw| {
        let txn_cloned = Arc::clone(&shared_txn);
        async move {
            let new_rule = RuleActiveModel {
                system_id: Set(rule_raw.system_id),
                attribute_rule: Set(rule_raw.attribute_rule),
                ..Default::default()
            };
            let created_rule = new_rule.insert(*txn_cloned).await?;

            let clauses_to_create = rule_raw
                .clauses
                .into_iter()
                .map(|clause| ClauseModel {
                    id: -1,
                    rule_id: created_rule.id,
                    compared_value: clause.compared_value,
                    logical_group: clause.logical_group,
                    operator: clause.operator,
                    question_id: clause.question_id,
                })
                .collect();
            let answers_to_create = rule_raw
                .rule_question_answer_ids
                .into_iter()
                .map(|clause| RuleQuestionAnswerModel {
                    id: -1,
                    rule_id: created_rule.id,
                    question_id: clause.question_id,
                    answer_id: clause.answer_id,
                })
                .collect();
            let attributevalues_to_create = rule_raw
                .rule_attribute_attributevalue_ids
                .into_iter()
                .map(|attributevalue| RuleAttributeAttributeValueModel {
                    id: -1,
                    rule_id: created_rule.id,
                    attribute_id: attributevalue.attribute_id,
                    attribute_value_id: attributevalue.attribute_value_id,
                })
                .collect();

            let (clauses, rule_question_answers, rule_attribute_attributevalues) = try_join!(
                create_clauses(*txn_cloned, clauses_to_create),
                create_rule_question_answers(*txn_cloned, answers_to_create),
                create_rule_attribute_attributevalues(*txn_cloned, attributevalues_to_create)
            )?;

            Ok::<RuleWithClausesAndEffects, DbErr>(RuleWithClausesAndEffects {
                id: created_rule.id,
                system_id: created_rule.system_id,
                attribute_rule: created_rule.attribute_rule,
                clauses,
                rule_question_answer_ids: rule_question_answers,
                rule_attribute_attributevalue_ids: rule_attribute_attributevalues,
            })
        }
    });

    let mut result = try_join_all(new_rules).await?;
    result.sort_by_key(|rule| rule.id);

    txn.commit().await?;

    Ok(result)
}

pub async fn multiple_delete_rules<C>(db: &C, rules_ids: Vec<i32>) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(RuleEntity::delete_many()
        .filter(RuleColumn::Id.is_in(rules_ids))
        .exec(db)
        .await?
        .rows_affected)
}
