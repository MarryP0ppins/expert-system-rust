use std::collections::HashMap;

use crate::{
    models::{
        answer::{Answer, NewAnswer},
        attribute::{Attribute, NewAttribute},
        attribute_value::{AttributeValue, NewAttributeValue},
        clause::{Clause, NewClause},
        error::CustomErrors,
        object::{NewObject, Object},
        object_attribute_attributevalue::{
            NewObjectAttributeAttributevalue, ObjectAttributeAttributevalue,
        },
        question::{NewQuestion, Question},
        rule::{NewRule, Rule},
        rule_attribute_attributevalue::{
            NewRuleAttributeAttributeValue, RuleAttributeAttributeValue,
        },
        rule_question_answer::{NewRuleQuestionAnswer, RuleQuestionAnswer},
        system::{NewSystem, System},
    },
    schema::{
        answers, attributes, attributesvalues, clauses, object_attribute_attributevalue, objects,
        questions, rule_attribute_attributevalue, rule_question_answer, rules, systems::dsl::*,
    },
};
use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use http::StatusCode;

pub async fn copy_system(
    connection: &mut AsyncPgConnection,
    old_system: &System,
) -> Result<System, CustomErrors> {
    Ok(insert_into(systems)
        .values::<NewSystem>(NewSystem {
            user_id: old_system.user_id,
            about: old_system.about.clone(),
            name: format!("{} - {}", old_system.name, chrono::Utc::now()),
            image_uri: old_system.image_uri.clone(),
            private: old_system.private,
        })
        .get_result::<System>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?)
}

pub async fn copy_questions(
    connection: &mut AsyncPgConnection,
    new_system_id: i32,
    old_questions: &Vec<Question>,
    question_map: &mut HashMap<i32, i32>,
) -> Result<Vec<Question>, CustomErrors> {
    let new_questions = insert_into(questions::table)
        .values::<Vec<NewQuestion>>(
            old_questions
                .as_slice()
                .into_iter()
                .map(|question| NewQuestion {
                    system_id: new_system_id,
                    body: question.body.clone(),
                    with_chooses: question.with_chooses,
                })
                .collect::<Vec<NewQuestion>>(),
        )
        .get_results::<Question>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    question_map.extend(
        old_questions
            .into_iter()
            .zip(&new_questions)
            .map(|(old_question, new_question)| (old_question.id, new_question.id)),
    );

    Ok(new_questions)
}

pub async fn copy_attributes(
    connection: &mut AsyncPgConnection,
    new_system_id: i32,
    old_attributes: &Vec<Attribute>,
    attribute_map: &mut HashMap<i32, i32>,
) -> Result<Vec<Attribute>, CustomErrors> {
    let new_attributes = insert_into(attributes::table)
        .values::<Vec<NewAttribute>>(
            old_attributes
                .as_slice()
                .into_iter()
                .map(|attribute| NewAttribute {
                    system_id: new_system_id,
                    name: attribute.name.clone(),
                })
                .collect::<Vec<NewAttribute>>(),
        )
        .get_results::<Attribute>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    attribute_map.extend(
        old_attributes
            .into_iter()
            .zip(&new_attributes)
            .map(|(old_question, new_question)| (old_question.id, new_question.id)),
    );

    Ok(new_attributes)
}

pub async fn copy_objects(
    connection: &mut AsyncPgConnection,
    new_system_id: i32,
    old_objects: &Vec<Object>,
    object_map: &mut HashMap<i32, i32>,
) -> Result<Vec<Object>, CustomErrors> {
    let new_objects = insert_into(objects::table)
        .values::<Vec<NewObject>>(
            old_objects
                .as_slice()
                .into_iter()
                .map(|object| NewObject {
                    system_id: new_system_id,
                    name: object.name.clone(),
                })
                .collect::<Vec<NewObject>>(),
        )
        .get_results::<Object>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    object_map.extend(
        old_objects
            .into_iter()
            .zip(&new_objects)
            .map(|(old_object, new_object)| (old_object.id, new_object.id)),
    );

    Ok(new_objects)
}

pub async fn copy_rules(
    connection: &mut AsyncPgConnection,
    new_system_id: i32,
    old_rules: &Vec<Rule>,
    rule_map: &mut HashMap<i32, i32>,
) -> Result<Vec<Rule>, CustomErrors> {
    let new_rules = insert_into(rules::table)
        .values::<Vec<NewRule>>(
            old_rules
                .as_slice()
                .into_iter()
                .map(|rule| NewRule {
                    system_id: new_system_id,
                    attribute_rule: rule.attribute_rule,
                })
                .collect::<Vec<NewRule>>(),
        )
        .get_results::<Rule>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    rule_map.extend(
        old_rules
            .into_iter()
            .zip(&new_rules)
            .map(|(old_rule, new_rule)| (old_rule.id, new_rule.id)),
    );

    Ok(new_rules)
}

pub async fn copy_attribute_values(
    connection: &mut AsyncPgConnection,
    old_attribute_values: &Vec<AttributeValue>,
    attribute_map: &HashMap<i32, i32>,
    attributevalue_map: &mut HashMap<i32, i32>,
) -> Result<Vec<AttributeValue>, CustomErrors> {
    let new_new_attribute_values: Result<Vec<NewAttributeValue>, CustomErrors> =
        old_attribute_values
            .into_iter()
            .map(|old_attribute_value| {
                let new_attribute_id = attribute_map
                    .get(&old_attribute_value.attribute_id)
                    .ok_or_else(|| CustomErrors::StringError {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        error: "Ошибка в расшифровке системы".to_string(),
                    })?;
                Ok(NewAttributeValue {
                    attribute_id: *new_attribute_id,
                    value: old_attribute_value.value.clone(),
                })
            })
            .collect();
    let new_new_attribute_values = match new_new_attribute_values {
        Ok(values) => values,
        Err(err) => return Err(err),
    };
    let new_attributevalues = insert_into(attributesvalues::table)
        .values(&new_new_attribute_values)
        .get_results::<AttributeValue>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    attributevalue_map.extend(
        old_attribute_values
            .into_iter()
            .zip(&new_attributevalues)
            .map(|(old_attributevalue, new_attributevalue)| {
                (old_attributevalue.id, new_attributevalue.id)
            }),
    );

    Ok(new_attributevalues)
}

pub async fn copy_answers(
    connection: &mut AsyncPgConnection,
    old_answers: &Vec<Answer>,
    question_map: &HashMap<i32, i32>,
    answer_map: &mut HashMap<i32, i32>,
) -> Result<Vec<Answer>, CustomErrors> {
    let new_new_answers: Result<Vec<NewAnswer>, CustomErrors> = old_answers
        .into_iter()
        .map(|old_answer| {
            let new_question_id = question_map.get(&old_answer.question_id).ok_or_else(|| {
                CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                }
            })?;
            Ok(NewAnswer {
                question_id: *new_question_id,
                body: old_answer.body.clone(),
            })
        })
        .collect();
    let new_new_answers = match new_new_answers {
        Ok(values) => values,
        Err(err) => return Err(err),
    };
    let new_answers = insert_into(answers::table)
        .values(&new_new_answers)
        .get_results::<Answer>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    answer_map.extend(
        old_answers
            .into_iter()
            .zip(&new_answers)
            .map(|(old_answer, new_answer)| (old_answer.id, new_answer.id)),
    );

    Ok(new_answers)
}

pub async fn copy_clauses(
    connection: &mut AsyncPgConnection,
    old_clauses: &Vec<Clause>,
    rule_map: &HashMap<i32, i32>,
    question_map: &HashMap<i32, i32>,
) -> Result<Vec<Clause>, CustomErrors> {
    let new_new_clauses: Result<Vec<NewClause>, CustomErrors> = old_clauses
        .into_iter()
        .map(|old_clause| {
            let new_rule_id =
                rule_map
                    .get(&old_clause.rule_id)
                    .ok_or_else(|| CustomErrors::StringError {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        error: "Ошибка в расшифровке системы".to_string(),
                    })?;
            let new_question_id = question_map.get(&old_clause.question_id).ok_or_else(|| {
                CustomErrors::StringError {
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    error: "Ошибка в расшифровке системы".to_string(),
                }
            })?;
            Ok(NewClause {
                rule_id: *new_rule_id,
                compared_value: old_clause.compared_value.clone(),
                logical_group: old_clause.logical_group.clone(),
                operator: old_clause.operator.clone(),
                question_id: *new_question_id,
            })
        })
        .collect();
    let new_new_clauses = match new_new_clauses {
        Ok(values) => values,
        Err(err) => return Err(err),
    };
    let new_clauses = insert_into(clauses::table)
        .values(&new_new_clauses)
        .get_results::<Clause>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    Ok(new_clauses)
}

pub async fn copy_rule_attribute_attributevalues(
    connection: &mut AsyncPgConnection,
    old_rule_attribute_attributevalues: &Vec<RuleAttributeAttributeValue>,
    rule_map: &HashMap<i32, i32>,
    attribute_map: &HashMap<i32, i32>,
    attributevalue_map: &HashMap<i32, i32>,
) -> Result<Vec<RuleAttributeAttributeValue>, CustomErrors> {
    let new_new_rule_attribute_attributevalue: Result<
        Vec<NewRuleAttributeAttributeValue>,
        CustomErrors,
    > = old_rule_attribute_attributevalues
        .into_iter()
        .map(|old_rule_attribute_attributevalue| {
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
            Ok(NewRuleAttributeAttributeValue {
                rule_id: *new_rule_id,
                attribute_value_id: *new_attribute_value_id,
                attribute_id: *new_attribute_id,
            })
        })
        .collect();
    let new_new_rule_attribute_attributevalue = match new_new_rule_attribute_attributevalue {
        Ok(values) => values,
        Err(err) => return Err(err),
    };
    let new_rule_attribute_attributevalues = insert_into(rule_attribute_attributevalue::table)
        .values(&new_new_rule_attribute_attributevalue)
        .get_results::<RuleAttributeAttributeValue>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    Ok(new_rule_attribute_attributevalues)
}

pub async fn copy_rule_question_answers(
    connection: &mut AsyncPgConnection,
    old_rule_question_answers: &Vec<RuleQuestionAnswer>,
    rule_map: &HashMap<i32, i32>,
    answer_map: &HashMap<i32, i32>,
    question_map: &HashMap<i32, i32>,
) -> Result<Vec<RuleQuestionAnswer>, CustomErrors> {
    let new_new_rule_question_answer: Result<Vec<NewRuleQuestionAnswer>, CustomErrors> =
        old_rule_question_answers
            .into_iter()
            .map(|old_rule_question_answer| {
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
                Ok(NewRuleQuestionAnswer {
                    rule_id: *new_rule_id,
                    answer_id: *new_answer_id,
                    question_id: *new_question_id,
                })
            })
            .collect();
    let new_new_rule_question_answer = match new_new_rule_question_answer {
        Ok(values) => values,
        Err(err) => return Err(err),
    };
    let new_rule_question_answers = insert_into(rule_question_answer::table)
        .values(&new_new_rule_question_answer)
        .get_results::<RuleQuestionAnswer>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    Ok(new_rule_question_answers)
}

pub async fn copy_object_attribute_attributevalues(
    connection: &mut AsyncPgConnection,
    old_object_attribute_attributevalues: &Vec<ObjectAttributeAttributevalue>,
    object_map: &HashMap<i32, i32>,
    attribute_map: &HashMap<i32, i32>,
    attributevalue_map: &HashMap<i32, i32>,
) -> Result<Vec<ObjectAttributeAttributevalue>, CustomErrors> {
    let new_new_object_attribute_attributevalue: Result<
        Vec<NewObjectAttributeAttributevalue>,
        CustomErrors,
    > = old_object_attribute_attributevalues
        .into_iter()
        .map(|old_object_attribute_attributevalue| {
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
            Ok(NewObjectAttributeAttributevalue {
                object_id: *new_object_id,
                attribute_value_id: *new_attribute_value_id,
                attribute_id: *new_attribute_id,
            })
        })
        .collect();
    let new_new_object_attribute_attributevalue = match new_new_object_attribute_attributevalue {
        Ok(values) => values,
        Err(err) => return Err(err),
    };
    let new_object_attribute_attributevalues = insert_into(object_attribute_attributevalue::table)
        .values(&new_new_object_attribute_attributevalue)
        .get_results::<ObjectAttributeAttributevalue>(connection)
        .await
        .map_err(|err| CustomErrors::DieselError {
            error: err,
            message: None,
        })?;

    Ok(new_object_attribute_attributevalues)
}
