use crate::{
    error::CustomErrors,
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
use entity::{
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
};
use http::StatusCode;
use sea_orm::{ConnectionTrait, EntityTrait, LoaderTrait, ModelTrait, TransactionTrait};
use std::collections::HashMap;
use tokio::try_join;
use tower_cookies::{Cookies, Key};

pub async fn backup_from_system<C>(
    db: &C,
    system_id: i32,
    crypto_key: &[u8],
    nonce_key: &[u8],
) -> Result<Vec<u8>, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let system = SystemEntity::find_by_id(system_id)
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

    let (objects, attributes, rules, questions) = try_join!(
        system.find_related(ObjectEntity).all(db),
        system.find_related(AttributeEntity).all(db),
        system.find_related(RuleEntity).all(db),
        system.find_related(QuestionEntity).all(db),
    )
    .map_err(|err| CustomErrors::SeaORMError {
        error: err,
        message: None,
    })?;

    let (
        object_attribute_values,
        attribute_values,
        rule_attribute_values,
        clause_entities,
        rule_question_answers,
        answers,
    ) = try_join!(
        objects.load_many(ObjectAttributeAttributeValueEntity, db),
        attributes.load_many(AttributeValueEntity, db),
        attributes.load_many(RuleAttributeAttributeValueEntity, db),
        questions.load_many(ClauseEntity, db),
        questions.load_many(RuleQuestionAnswerEntity, db),
        questions.load_many(AnswerEntity, db),
    )
    .map_err(|err| CustomErrors::SeaORMError {
        error: err,
        message: None,
    })?;

    let struct_to_encrypt = SystemBackupModel {
        system,
        objects,
        object_attribute_attributevalue: object_attribute_values
            .into_iter()
            .flat_map(|arr| arr)
            .collect(),
        attributes,
        attributes_values: attribute_values.into_iter().flat_map(|arr| arr).collect(),
        rules,
        rule_attribute_attributevalue: rule_attribute_values
            .into_iter()
            .flat_map(|arr| arr)
            .collect(),
        clauses: clause_entities.into_iter().flat_map(|arr| arr).collect(),
        questions,
        answers: answers.into_iter().flat_map(|arr| arr).collect(),
        rule_question_answer: rule_question_answers
            .into_iter()
            .flat_map(|arr| arr)
            .collect(),
    };
    let encoded: Vec<u8> = bincode::serialize(&struct_to_encrypt).expect("serialize error");

    let encrypt_backup =
        encrypt_data(crypto_key, nonce_key, &encoded).map_err(|err| CustomErrors::AesGsmError {
            error: err,
            message: Some("Ошибка в создании резервной копии".to_string()),
        })?;

    Ok(encrypt_backup)
}

pub async fn system_from_backup<C>(
    db: &C,
    encrypted_system: Vec<u8>,
    cookie: Cookies,
    cookie_key: &Key,
    crypto_key: &[u8],
    nonce_key: &[u8],
) -> Result<SystemModel, CustomErrors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let decoded_system = decrypt_data(crypto_key, nonce_key, &encrypted_system).map_err(|err| {
        CustomErrors::AesGsmError {
            error: err,
            message: Some("Файл поврежден или изменен".to_string()),
        }
    })?;

    let system_backup: SystemBackupModel =
        bincode::deserialize(&decoded_system).map_err(|_| CustomErrors::StringError {
            status: StatusCode::BAD_REQUEST,
            error: "Ошибка декодирования".to_string(),
        })?;

    let user_cookie = cookie_check(db, cookie, cookie_key).await?;
    if user_cookie.id != system_backup.system.user_id {
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

    let new_system = copy_system(&txn, &system_backup.system).await?;
    let new_system_id = new_system.id;

    try_join!(
        copy_questions(
            &txn,
            new_system_id,
            &system_backup.questions,
            &mut question_map
        ),
        copy_attributes(
            &txn,
            new_system_id,
            &system_backup.attributes,
            &mut attribute_map
        ),
        copy_objects(&txn, new_system_id, &system_backup.objects, &mut object_map),
        copy_rules(&txn, new_system_id, &system_backup.rules, &mut rule_map)
    )?;

    try_join!(
        copy_attribute_values(
            &txn,
            &system_backup.attributes_values,
            &attribute_map,
            &mut attributevalue_map,
        ),
        copy_answers(&txn, &system_backup.answers, &question_map, &mut answer_map)
    )?;

    try_join!(
        copy_clauses(&txn, &system_backup.clauses, &rule_map, &question_map),
        copy_rule_attribute_attributevalues(
            &txn,
            &system_backup.rule_attribute_attributevalue,
            &rule_map,
            &attribute_map,
            &attributevalue_map,
        ),
        copy_rule_question_answers(
            &txn,
            &system_backup.rule_question_answer,
            &rule_map,
            &answer_map,
            &question_map,
        ),
        copy_object_attribute_attributevalues(
            &txn,
            &system_backup.object_attribute_attributevalue,
            &object_map,
            &attribute_map,
            &attributevalue_map,
        )
    )?;

    txn.commit()
        .await
        .map_err(|err| CustomErrors::SeaORMError {
            error: err,
            message: None,
        })?;

    return Ok(new_system);
}
