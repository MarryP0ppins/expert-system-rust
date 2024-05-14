use crate::{
    models::{
        answer::Answer,
        attribute::Attribute,
        attribute_value::AttributeValue,
        clause::Clause,
        error::CustomErrors,
        object::Object,
        object_attribute_attributevalue::ObjectAttributeAttributevalue,
        question::Question,
        rule::Rule,
        rule_attribute_attributevalue::RuleAttributeAttributeValue,
        rule_question_answer::RuleQuestionAnswer,
        system::{System, SystemBackup},
    },
    schema::{
        answers, attributes, attributesvalues, clauses, object_attribute_attributevalue, objects,
        questions, rule_attribute_attributevalue, rule_question_answer, rules, systems::dsl::*,
    },
    utils::crypto::encrypt_data,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn backup_from_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<Vec<u8>, CustomErrors> {
    // ----------SYSTEM----------
    let _system;
    match systems.find(system_id).first::<System>(connection).await {
        Ok(result) => _system = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------OBJECTS----------
    let _objects;
    match objects::table
        .filter(objects::system_id.eq(system_id))
        .load::<Object>(connection)
        .await
    {
        Ok(result) => _objects = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    let _objects_ids = _objects
        .as_slice()
        .into_iter()
        .map(|object| object.id)
        .collect::<Vec<i32>>();
    // ----------ATTRIBUTES_VALUE_OBJECTS----------
    let _object_attribute_attributevalue;
    match object_attribute_attributevalue::table
        .filter(object_attribute_attributevalue::object_id.eq_any(_objects_ids))
        .load::<ObjectAttributeAttributevalue>(connection)
        .await
    {
        Ok(result) => _object_attribute_attributevalue = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------ATTRIBUTES----------
    let _attributes;
    match attributes::table
        .filter(attributes::system_id.eq(system_id))
        .load::<Attribute>(connection)
        .await
    {
        Ok(result) => _attributes = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    let _attributes_ids = _attributes
        .as_slice()
        .into_iter()
        .map(|attribute| attribute.id)
        .collect::<Vec<i32>>();
    // ----------ATTRIBUTES_VALUES----------
    let _attributes_values;
    match attributesvalues::table
        .filter(attributesvalues::attribute_id.eq_any(&_attributes_ids))
        .load::<AttributeValue>(connection)
        .await
    {
        Ok(result) => _attributes_values = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------ATTRIBUTES_VALUE_OBJECTS----------
    let _rule_attribute_attributevalue;
    match rule_attribute_attributevalue::table
        .filter(rule_attribute_attributevalue::attribute_id.eq_any(_attributes_ids))
        .load::<RuleAttributeAttributeValue>(connection)
        .await
    {
        Ok(result) => _rule_attribute_attributevalue = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------RULES----------
    let _rules;
    match rules::table
        .filter(rules::system_id.eq(system_id))
        .load::<Rule>(connection)
        .await
    {
        Ok(result) => _rules = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------QUESTIONS----------
    let _questions;
    match questions::table
        .filter(questions::system_id.eq(system_id))
        .load::<Question>(connection)
        .await
    {
        Ok(result) => _questions = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    let _questions_ids = _questions
        .as_slice()
        .into_iter()
        .map(|question| question.id)
        .collect::<Vec<i32>>();
    // ----------CLAUSES----------
    let _clauses;
    match clauses::table
        .filter(clauses::question_id.eq_any(_questions_ids.clone()))
        .load::<Clause>(connection)
        .await
    {
        Ok(result) => _clauses = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------RULE_QUESTION_ANSWER----------
    let _rules_question_answer;
    match rule_question_answer::table
        .filter(rule_question_answer::question_id.eq_any(_questions_ids.clone()))
        .load::<RuleQuestionAnswer>(connection)
        .await
    {
        Ok(result) => _rules_question_answer = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------ANSWERS----------
    let _answers;
    match answers::table
        .filter(answers::question_id.eq_any(_questions_ids.clone()))
        .load::<Answer>(connection)
        .await
    {
        Ok(result) => _answers = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    let struct_to_encrypt = SystemBackup {
        system: _system,
        objects: _objects,
        object_attribute_attributevalue: _object_attribute_attributevalue,
        attributes: _attributes,
        attributes_values: _attributes_values,
        rules: _rules,
        rule_attribute_attributevalue: _rule_attribute_attributevalue,
        clauses: _clauses,
        questions: _questions,
        answers: _answers,
        rules_question_answer: _rules_question_answer,
    };
    let encoded: Vec<u8> = bincode::serialize(&struct_to_encrypt).expect("serialize error");

    let _encrypt_key: &[u8] = dotenv!("CRYPTO_KEY").as_bytes();
    let encrypt_backup;
    match encrypt_data(_encrypt_key, &encoded) {
        Ok(result) => encrypt_backup = result,
        Err(err) => {
            return Err(CustomErrors::AesGsmError {
                error: err,
                message: Some("Ошибка в создании резервной копии".to_string()),
            })
        }
    };

    Ok(encrypt_backup)
}
