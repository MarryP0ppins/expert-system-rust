use std::collections::HashMap;

use crate::{
    entity::{
        answers::{ActiveModel as AnswerActiveModel, Model as AnswerModel},
        attributes::{ActiveModel as AttributeActiveModel, Model as AttributeModel},
        attributesvalues::{
            ActiveModel as AttributeValueActiveModel, Model as AttributeValueModel,
        },
        clauses::{ActiveModel as ClauseActiveModel, Model as ClauseModel},
        object_attribute_attributevalue::{
            ActiveModel as ObjectAttributeAttributeValueActiveModel,
            Model as ObjectAttributeAttributeValueModel,
        },
        objects::{ActiveModel as ObjectActiveModel, Model as ObjectModel},
        questions::{ActiveModel as QuestionActiveModel, Model as QuestionModel},
        rule_attribute_attributevalue::{
            ActiveModel as RuleAttributeAttributeValueActiveModel,
            Model as RuleAttributeAttributeValueModel,
        },
        rule_question_answer::{
            ActiveModel as RuleQuestionAnswerActiveModel, Model as RuleQuestionAnswerModel,
        },
        rules::{ActiveModel as RuleActiveModel, Model as RuleModel},
        systems::{ActiveModel as SystemActiveModel, Model as SystemModel},
    },
    models::error::CustomErrors,
};

use futures::future::try_join_all;
use http::StatusCode;
use sea_orm::*;

pub async fn copy_system<C>(db: &C, old_system: &SystemModel) -> Result<SystemModel, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut split_name = old_system.name.clone();
    let _ = split_name.split_off(94);

    let model = SystemActiveModel {
        user_id: Set(old_system.user_id),
        about: Set(old_system.about.clone()),
        name: Set(format!("{} - {}", split_name, chrono::Utc::now())),
        private: Set(old_system.private),
        image_uri: Set(old_system.image_uri.clone()),
        ..Default::default()
    };
    Ok(model
        .insert(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?)
}

pub async fn copy_questions<C>(
    db: &C,
    new_system_id: i32,
    old_questions: &Vec<QuestionModel>,
    question_map: &mut HashMap<i32, i32>,
) -> Result<Vec<QuestionModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_questions = old_questions.into_iter().map(|question| async move {
        let model = QuestionActiveModel {
            system_id: Set(new_system_id),
            body: Set(question.body.clone()),
            with_chooses: Set(question.with_chooses),
            ..Default::default()
        };
        model
            .insert(db)
            .await
            .map_err(|err| CustomErrors::SeaORMError {
                error: err,
                message: None,
            })
    });
    let result = try_join_all(new_questions).await?;

    question_map.extend(
        old_questions
            .into_iter()
            .zip(&result)
            .map(|(old_question, new_question)| (old_question.id, new_question.id)),
    );

    Ok(result)
}

pub async fn copy_attributes<C>(
    db: &C,
    new_system_id: i32,
    old_attributes: &Vec<AttributeModel>,
    attribute_map: &mut HashMap<i32, i32>,
) -> Result<Vec<AttributeModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_attributes = old_attributes.into_iter().map(|attribute| async move {
        let model = AttributeActiveModel {
            system_id: Set(new_system_id),
            name: Set(attribute.name.clone()),
            ..Default::default()
        };
        model
            .insert(db)
            .await
            .map_err(|err| CustomErrors::SeaORMError {
                error: err,
                message: None,
            })
    });
    let result = try_join_all(new_attributes).await?;

    attribute_map.extend(
        old_attributes
            .into_iter()
            .zip(&result)
            .map(|(old_question, new_question)| (old_question.id, new_question.id)),
    );

    Ok(result)
}

pub async fn copy_objects<C>(
    db: &C,
    new_system_id: i32,
    old_objects: &Vec<ObjectModel>,
    object_map: &mut HashMap<i32, i32>,
) -> Result<Vec<ObjectModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_objects = old_objects.into_iter().map(|object| async move {
        let model = ObjectActiveModel {
            system_id: Set(new_system_id),
            name: Set(object.name.clone()),
            ..Default::default()
        };
        model
            .insert(db)
            .await
            .map_err(|err| CustomErrors::SeaORMError {
                error: err,
                message: None,
            })
    });
    let result = try_join_all(new_objects).await?;

    object_map.extend(
        old_objects
            .into_iter()
            .zip(&result)
            .map(|(old_object, new_object)| (old_object.id, new_object.id)),
    );

    Ok(result)
}

pub async fn copy_rules<C>(
    db: &C,
    new_system_id: i32,
    old_rules: &Vec<RuleModel>,
    rule_map: &mut HashMap<i32, i32>,
) -> Result<Vec<RuleModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_rules = old_rules.into_iter().map(|rule| async move {
        let model = RuleActiveModel {
            system_id: Set(new_system_id),
            attribute_rule: Set(rule.attribute_rule),
            ..Default::default()
        };
        model
            .insert(db)
            .await
            .map_err(|err| CustomErrors::SeaORMError {
                error: err,
                message: None,
            })
    });
    let result = try_join_all(new_rules).await?;

    rule_map.extend(
        old_rules
            .into_iter()
            .zip(&result)
            .map(|(old_rule, new_rule)| (old_rule.id, new_rule.id)),
    );

    Ok(result)
}

pub async fn copy_attribute_values<C>(
    db: &C,
    old_attribute_values: &Vec<AttributeValueModel>,
    attribute_map: &HashMap<i32, i32>,
    attributevalue_map: &mut HashMap<i32, i32>,
) -> Result<Vec<AttributeValueModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_new_attribute_values =
        old_attribute_values
            .into_iter()
            .map(|old_attribute_value| async move {
                let new_attribute_id = attribute_map.get(&old_attribute_value.attribute_id).ok_or(
                    CustomErrors::StringError {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        error: "Ошибка в расшифровке системы".to_string(),
                    },
                )?;
                let model = AttributeValueActiveModel {
                    attribute_id: Set(*new_attribute_id),
                    value: Set(old_attribute_value.value.clone()),
                    ..Default::default()
                };
                model
                    .insert(db)
                    .await
                    .map_err(|err| CustomErrors::SeaORMError {
                        error: err,
                        message: None,
                    })
            });
    let result = try_join_all(new_new_attribute_values).await?;

    attributevalue_map.extend(old_attribute_values.into_iter().zip(&result).map(
        |(old_attributevalue, new_attributevalue)| (old_attributevalue.id, new_attributevalue.id),
    ));

    Ok(result)
}

pub async fn copy_answers<C>(
    db: &C,
    old_answers: &Vec<AnswerModel>,
    question_map: &HashMap<i32, i32>,
    answer_map: &mut HashMap<i32, i32>,
) -> Result<Vec<AnswerModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_new_answers = old_answers.into_iter().map(|old_answer| async move {
        let new_question_id =
            question_map
                .get(&old_answer.question_id)
                .ok_or(CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
        let model = AnswerActiveModel {
            question_id: Set(*new_question_id),
            body: Set(old_answer.body.clone()),
            ..Default::default()
        };
        model
            .insert(db)
            .await
            .map_err(|err| CustomErrors::SeaORMError {
                error: err,
                message: None,
            })
    });
    let result = try_join_all(new_new_answers).await?;

    answer_map.extend(
        old_answers
            .into_iter()
            .zip(&result)
            .map(|(old_answer, new_answer)| (old_answer.id, new_answer.id)),
    );

    Ok(result)
}

pub async fn copy_clauses<C>(
    db: &C,
    old_clauses: &Vec<ClauseModel>,
    rule_map: &HashMap<i32, i32>,
    question_map: &HashMap<i32, i32>,
) -> Result<Vec<ClauseModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_new_clauses = old_clauses.into_iter().map(|old_clause| async move {
        let new_rule_id = rule_map
            .get(&old_clause.rule_id)
            .ok_or(CustomErrors::StringError {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                error: "Ошибка в расшифровке системы".to_string(),
            })?;
        let new_question_id =
            question_map
                .get(&old_clause.question_id)
                .ok_or(CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
        let model = ClauseActiveModel {
            rule_id: Set(*new_rule_id),
            compared_value: Set(old_clause.compared_value.clone()),
            logical_group: Set(old_clause.logical_group.clone()),
            operator: Set(old_clause.operator.clone()),
            question_id: Set(*new_question_id),
            ..Default::default()
        };
        model
            .insert(db)
            .await
            .map_err(|err| CustomErrors::SeaORMError {
                error: err,
                message: None,
            })
    });
    let result = try_join_all(new_new_clauses).await?;

    Ok(result)
}

pub async fn copy_rule_attribute_attributevalues<C>(
    db: &C,
    old_rule_attribute_attributevalues: &Vec<RuleAttributeAttributeValueModel>,
    rule_map: &HashMap<i32, i32>,
    attribute_map: &HashMap<i32, i32>,
    attributevalue_map: &HashMap<i32, i32>,
) -> Result<Vec<RuleAttributeAttributeValueModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_new_rule_attribute_attributevalue = old_rule_attribute_attributevalues.into_iter().map(
        |old_rule_attribute_attributevalue| async move {
            let new_rule_id = rule_map
                .get(&old_rule_attribute_attributevalue.rule_id)
                .ok_or_else(|| CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
            let new_attribute_id = attribute_map
                .get(&old_rule_attribute_attributevalue.attribute_id)
                .ok_or_else(|| CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
            let new_attribute_value_id = attributevalue_map
                .get(&old_rule_attribute_attributevalue.attribute_value_id)
                .ok_or_else(|| CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
            let model = RuleAttributeAttributeValueActiveModel {
                rule_id: Set(*new_rule_id),
                attribute_value_id: Set(*new_attribute_value_id),
                attribute_id: Set(*new_attribute_id),
                ..Default::default()
            };
            model
                .insert(db)
                .await
                .map_err(|err| CustomErrors::SeaORMError {
                    error: err,
                    message: None,
                })
        },
    );
    let result = try_join_all(new_new_rule_attribute_attributevalue).await?;

    Ok(result)
}

pub async fn copy_rule_question_answers<C>(
    db: &C,
    old_rule_question_answers: &Vec<RuleQuestionAnswerModel>,
    rule_map: &HashMap<i32, i32>,
    answer_map: &HashMap<i32, i32>,
    question_map: &HashMap<i32, i32>,
) -> Result<Vec<RuleQuestionAnswerModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_new_rule_question_answer =
        old_rule_question_answers
            .into_iter()
            .map(|old_rule_question_answer| async move {
                let new_rule_id =
                    rule_map
                        .get(&old_rule_question_answer.rule_id)
                        .ok_or_else(|| CustomErrors::StringError {
                            status: StatusCode::UNPROCESSABLE_ENTITY,
                            error: "Ошибка в расшифровке системы".to_string(),
                        })?;
                let new_question_id = question_map
                    .get(&old_rule_question_answer.question_id)
                    .ok_or_else(|| CustomErrors::StringError {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        error: "Ошибка в расшифровке системы".to_string(),
                    })?;
                let new_answer_id = answer_map
                    .get(&old_rule_question_answer.answer_id)
                    .ok_or_else(|| CustomErrors::StringError {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        error: "Ошибка в расшифровке системы".to_string(),
                    })?;
                let model = RuleQuestionAnswerActiveModel {
                    rule_id: Set(*new_rule_id),
                    answer_id: Set(*new_answer_id),
                    question_id: Set(*new_question_id),
                    ..Default::default()
                };
                model
                    .insert(db)
                    .await
                    .map_err(|err| CustomErrors::SeaORMError {
                        error: err,
                        message: None,
                    })
            });
    let result = try_join_all(new_new_rule_question_answer).await?;

    Ok(result)
}

pub async fn copy_object_attribute_attributevalues<C>(
    db: &C,
    old_object_attribute_attributevalues: &Vec<ObjectAttributeAttributeValueModel>,
    object_map: &HashMap<i32, i32>,
    attribute_map: &HashMap<i32, i32>,
    attributevalue_map: &HashMap<i32, i32>,
) -> Result<Vec<ObjectAttributeAttributeValueModel>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_new_object_attribute_attributevalue = old_object_attribute_attributevalues
        .into_iter()
        .map(|old_object_attribute_attributevalue| async move {
            let new_object_id = object_map
                .get(&old_object_attribute_attributevalue.object_id)
                .ok_or_else(|| CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
            let new_attribute_id = attribute_map
                .get(&old_object_attribute_attributevalue.attribute_id)
                .ok_or_else(|| CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
            let new_attribute_value_id = attributevalue_map
                .get(&old_object_attribute_attributevalue.attribute_value_id)
                .ok_or_else(|| CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                })?;
            let model = ObjectAttributeAttributeValueActiveModel {
                object_id: Set(*new_object_id),
                attribute_value_id: Set(*new_attribute_value_id),
                attribute_id: Set(*new_attribute_id),
                ..Default::default()
            };
            model
                .insert(db)
                .await
                .map_err(|err| CustomErrors::SeaORMError {
                    error: err,
                    message: None,
                })
        });
    let result = try_join_all(new_new_object_attribute_attributevalue).await?;

    Ok(result)
}
