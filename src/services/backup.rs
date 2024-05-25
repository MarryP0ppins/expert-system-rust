use std::collections::HashMap;

use crate::{
    entity::{
        answers::Entity as AnswerEntity,
        attributes::Entity as AttributeEntity,
        attributesvalues::Entity as AttributeValueEntity,
        clauses::Entity as ClauseEntity,
        object_attribute_attributevalue::Entity as ObjectAttributeAttributeValueEntity,
        objects::Entity as ObjectEntity,
        questions::Entity as QuestionEntity,
        rule_attribute_attributevalue::Entity as RuleAttributeAttributeValueEntity,
        rule_question_answer::Entity as RuleQuestionAnswerEntity,
        rules::Entity as RuleEntity,
        systems::{Entity as SystemEntity, Model as SystemModel, SystemBackupModel},
    },
    models::error::CustomErrors,
    utils::{
        auth::cookie_check,
        copy::{
            copy_answers, copy_attribute_values, copy_attributes, copy_clauses,
            copy_object_attribute_attributevalues, copy_objects, copy_questions,
            copy_rule_attribute_attributevalues, copy_rule_question_answers, copy_rules,
            copy_system,
        },
        crypto::{decrypt_data, encrypt_data},
    },
};
use http::StatusCode;
use sea_orm::*;
use tower_cookies::{Cookies, Key};

pub async fn backup_from_system<C>(db: &C, system_id: i32) -> Result<Vec<u8>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    // ----------SYSTEM----------
    let _system = SystemEntity::find_by_id(system_id)
        .one(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?
        .ok_or(CustomErrors::StringError {
            status: StatusCode::BAD_REQUEST,
            error: "Система не найдена".to_string(),
        })?;
    // ----------OBJECTS----------
    let _objects = _system
        .find_related(ObjectEntity)
        .all(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------ATTRIBUTES_VALUE_OBJECTS----------
    let _object_attribute_attributevalue = _objects
        .load_many(ObjectAttributeAttributeValueEntity, db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------ATTRIBUTES----------
    let _attributes = _system
        .find_related(AttributeEntity)
        .all(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------ATTRIBUTES_VALUES----------
    let _attributes_values = _attributes
        .load_many(AttributeValueEntity, db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------ATTRIBUTES_VALUE_OBJECTS----------
    let _rule_attribute_attributevalue = _attributes
        .load_many(RuleAttributeAttributeValueEntity, db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------RULES----------
    let _rules = _system
        .find_related(RuleEntity)
        .all(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------QUESTIONS----------
    let _questions = _system
        .find_related(QuestionEntity)
        .all(db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------CLAUSES----------
    let _clauses = _questions
        .load_many(ClauseEntity, db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------RULE_QUESTION_ANSWER----------
    let _rule_question_answer = _questions
        .load_many(RuleQuestionAnswerEntity, db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;
    // ----------ANSWERS----------
    let _answers = _questions
        .load_many(AnswerEntity, db)
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;

    let struct_to_encrypt = SystemBackupModel {
        system: _system,
        objects: _objects,
        object_attribute_attributevalue: _object_attribute_attributevalue
            .into_iter()
            .flat_map(|arr| arr)
            .collect(),
        attributes: _attributes,
        attributes_values: _attributes_values.into_iter().flat_map(|arr| arr).collect(),
        rules: _rules,
        rule_attribute_attributevalue: _rule_attribute_attributevalue
            .into_iter()
            .flat_map(|arr| arr)
            .collect(),
        clauses: _clauses.into_iter().flat_map(|arr| arr).collect(),
        questions: _questions,
        answers: _answers.into_iter().flat_map(|arr| arr).collect(),
        rule_question_answer: _rule_question_answer
            .into_iter()
            .flat_map(|arr| arr)
            .collect(),
    };
    let encoded: Vec<u8> = bincode::serialize(&struct_to_encrypt).expect("serialize error");

    let _crypt_key: &[u8] = dotenv!("CRYPTO_KEY").as_bytes();
    let _nonce_key: &[u8] = dotenv!("NONCE_KEY").as_bytes();
    let encrypt_backup = encrypt_data(_crypt_key, _nonce_key, &encoded).map_err(|err| {
        CustomErrors::AesGsmError {
            error: err,
            message: Some("Ошибка в создании резервной копии".to_string()),
        }
    })?;

    Ok(encrypt_backup)
}

pub async fn system_from_backup<C>(
    db: &C,
    encryped_system: Vec<u8>,
    cookie: Cookies,
    cookie_key: &Key,
) -> Result<SystemModel, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let _crypt_key: &[u8] = dotenv!("CRYPTO_KEY").as_bytes();
    let _nonce_key: &[u8] = dotenv!("NONCE_KEY").as_bytes();
    let decoded_system;

    match decrypt_data(_crypt_key, _nonce_key, &encryped_system) {
        Ok(result) => decoded_system = result,
        Err(err) => {
            return Err(CustomErrors::AesGsmError {
                error: err,
                message: Some("Файл поврежден или изменен".to_string()),
            })
        }
    };

    let _system: SystemBackupModel;
    match bincode::deserialize(&decoded_system) {
        Ok(system) => _system = system,
        Err(_) => {
            return Err(CustomErrors::StringError {
                status: StatusCode::BAD_REQUEST,
                error: "Ошибка декодирования".to_string(),
            })
        }
    };

    let user_cookie = cookie_check(db, cookie, cookie_key).await?;

    let mut new_system = _system.system.clone();

    if user_cookie.id != new_system.user_id {
        return Err(CustomErrors::StringError {
            status: StatusCode::BAD_REQUEST,
            error: "Чужая система".to_string(),
        });
    }

    let txn = db.begin().await.map_err(|err| CustomErrors::SeaORMError {
        error: err,
        message: None,
    })?;

    let mut question_map: HashMap<i32, i32> = HashMap::new();
    let mut attribute_map: HashMap<i32, i32> = HashMap::new();
    let mut object_map: HashMap<i32, i32> = HashMap::new();
    let mut rule_map: HashMap<i32, i32> = HashMap::new();
    let mut attributevalue_map: HashMap<i32, i32> = HashMap::new();
    let mut answer_map: HashMap<i32, i32> = HashMap::new();

    new_system = copy_system(&txn, &_system.system).await?;
    let new_system_id = new_system.id;

    copy_questions(&txn, new_system_id, &_system.questions, &mut question_map).await?;
    copy_attributes(&txn, new_system_id, &_system.attributes, &mut attribute_map).await?;
    copy_objects(&txn, new_system_id, &_system.objects, &mut object_map).await?;
    copy_rules(&txn, new_system_id, &_system.rules, &mut rule_map).await?;
    copy_attribute_values(
        &txn,
        &_system.attributes_values,
        &attribute_map,
        &mut attributevalue_map,
    )
    .await?;
    copy_answers(&txn, &_system.answers, &question_map, &mut answer_map).await?;
    copy_clauses(&txn, &_system.clauses, &rule_map, &question_map).await?;
    copy_rule_attribute_attributevalues(
        &txn,
        &_system.rule_attribute_attributevalue,
        &rule_map,
        &attribute_map,
        &attributevalue_map,
    )
    .await?;
    copy_rule_question_answers(
        &txn,
        &_system.rule_question_answer,
        &rule_map,
        &answer_map,
        &question_map,
    )
    .await?;
    copy_object_attribute_attributevalues(
        &txn,
        &_system.object_attribute_attributevalue,
        &object_map,
        &attribute_map,
        &attributevalue_map,
    )
    .await?;

    txn.commit()
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;

    return Ok(new_system);
}
