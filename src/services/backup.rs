use crate::{
    models::{
        answer::Answer,
        attribute::Attribute,
        attribute_value::AttributeValue,
        attribute_value_object::AttributeValueObject,
        clause::Clause,
        error::CustomErrors,
        object::Object,
        question::Question,
        rule::Rule,
        rule_answer::RuleAnswer,
        rule_attributevalue::RuleAttributeValue,
        system::{System, SystemBackup},
    },
    schema::{
        answers, attributes, attributesvalue_object, attributesvalues, clauses, objects, questions,
        rule_answer, rule_attributevalue, rules, systems::dsl::*,
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
    let _attributes_values_objects;
    match attributesvalue_object::table
        .filter(attributesvalue_object::object_id.eq_any(_objects_ids))
        .load::<AttributeValueObject>(connection)
        .await
    {
        Ok(result) => _attributes_values_objects = result,
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
    let _rule_attributes_values;
    match rule_attributevalue::table
        .filter(rule_attributevalue::attribute_id.eq_any(_attributes_ids))
        .load::<RuleAttributeValue>(connection)
        .await
    {
        Ok(result) => _rule_attributes_values = result,
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
    // ----------RULES_ANSWERS----------
    let _rules_answers;
    match rule_answer::table
        .filter(rule_answer::question_id.eq_any(_questions_ids.clone()))
        .load::<RuleAnswer>(connection)
        .await
    {
        Ok(result) => _rules_answers = result,
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
        attributes_values_objects: _attributes_values_objects,
        attributes: _attributes,
        attributes_values: _attributes_values,
        rules: _rules,
        rule_attributes_values: _rule_attributes_values,
        clauses: _clauses,
        questions: _questions,
        answers: _answers,
        rules_answers: _rules_answers,
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
