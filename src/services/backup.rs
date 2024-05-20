use std::collections::HashMap;

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
    schema::systems::dsl::*,
    utils::{
        copy::{
            copy_answers, copy_attribute_values, copy_attributes, copy_clauses,
            copy_object_attribute_attributevalues, copy_objects, copy_questions,
            copy_rule_attribute_attributevalues, copy_rule_question_answers, copy_rules,
            copy_system,
        },
        crypto::{decrypt_data, encrypt_data},
    },
};
use diesel::prelude::*;
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use http::StatusCode;

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
    match Object::belonging_to(&_system)
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
    // ----------ATTRIBUTES_VALUE_OBJECTS----------
    let _object_attribute_attributevalue;
    match ObjectAttributeAttributevalue::belonging_to(&_objects)
        .load::<ObjectAttributeAttributevalue>(connection)
        .await
    {
        Ok(ok) => _object_attribute_attributevalue = ok,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            });
        }
    };
    // ----------ATTRIBUTES----------
    let _attributes;
    match Attribute::belonging_to(&_system)
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
    // ----------ATTRIBUTES_VALUES----------
    let _attributes_values;
    match AttributeValue::belonging_to(&_attributes)
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
    match RuleAttributeAttributeValue::belonging_to(&_attributes)
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
    match Rule::belonging_to(&_system).load::<Rule>(connection).await {
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
    match Question::belonging_to(&_system)
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
    // ----------CLAUSES----------
    let _clauses;
    match Clause::belonging_to(&_questions)
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
    let _rule_question_answer;
    match RuleQuestionAnswer::belonging_to(&_questions)
        .load::<RuleQuestionAnswer>(connection)
        .await
    {
        Ok(result) => _rule_question_answer = result,
        Err(err) => {
            return Err(CustomErrors::DieselError {
                error: err,
                message: None,
            })
        }
    }
    // ----------ANSWERS----------
    let _answers;
    match Answer::belonging_to(&_questions)
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
        rule_question_answer: _rule_question_answer,
    };
    let encoded: Vec<u8> = bincode::serialize(&struct_to_encrypt).expect("serialize error");

    let _crypt_key: &[u8] = dotenv!("CRYPTO_KEY").as_bytes();
    let encrypt_backup;
    match encrypt_data(_crypt_key, &encoded) {
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

pub async fn system_from_backup(
    connection: &mut AsyncPgConnection,
    encryped_system: Vec<u8>,
) -> Result<(), CustomErrors> {
    let _crypt_key: &[u8] = dotenv!("CRYPTO_KEY").as_bytes();
    let decoded_system;
    match decrypt_data(_crypt_key, &encryped_system) {
        Ok(result) => decoded_system = result,
        Err(err) => {
            return Err(CustomErrors::AesGsmError {
                error: err,
                message: Some("Файл поврежден или изменен".to_string()),
            })
        }
    };

    let _system: SystemBackup;
    match bincode::deserialize(&decoded_system) {
        Ok(system) => _system = system,
        Err(err) => {
            println!("{err}");
            return Err(CustomErrors::StringError {
                status: StatusCode::BAD_REQUEST,
                error: "Ошибка декодирования".to_string(),
            });
        }
    };

    println!("{:?}", &_system);

    match connection
        .transaction(|connection| {
            async {
                let mut question_map: HashMap<i32, i32> = HashMap::new();
                let mut attribute_map: HashMap<i32, i32> = HashMap::new();
                let mut object_map: HashMap<i32, i32> = HashMap::new();
                let mut rule_map: HashMap<i32, i32> = HashMap::new();
                let mut attributevalue_map: HashMap<i32, i32> = HashMap::new();
                let mut answer_map: HashMap<i32, i32> = HashMap::new();

                let new_system = copy_system(connection, &_system.system).await?;
                let new_system_id = new_system.id;
                copy_questions(
                    connection,
                    new_system_id,
                    &_system.questions,
                    &mut question_map,
                )
                .await?;
                copy_attributes(
                    connection,
                    new_system_id,
                    &_system.attributes,
                    &mut attribute_map,
                )
                .await?;
                copy_objects(connection, new_system_id, &_system.objects, &mut object_map).await?;
                copy_rules(connection, new_system_id, &_system.rules, &mut rule_map).await?;
                copy_attribute_values(
                    connection,
                    &_system.attributes_values,
                    &attribute_map,
                    &mut attributevalue_map,
                )
                .await?;
                copy_answers(connection, &_system.answers, &question_map, &mut answer_map).await?;
                copy_clauses(connection, &_system.clauses, &rule_map, &question_map).await?;
                copy_rule_attribute_attributevalues(
                    connection,
                    &_system.rule_attribute_attributevalue,
                    &rule_map,
                    &attribute_map,
                    &attributevalue_map,
                )
                .await?;
                copy_rule_question_answers(
                    connection,
                    &_system.rule_question_answer,
                    &rule_map,
                    &answer_map,
                    &question_map,
                )
                .await?;
                copy_object_attribute_attributevalues(
                    connection,
                    &_system.object_attribute_attributevalue,
                    &object_map,
                    &attribute_map,
                    &attributevalue_map,
                )
                .await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    return Ok(());
}
