use crate::{
    pagination::SystemListPagination,
    services::{object::get_objects, rule::get_rules},
    utils::topological_sort::topological_sort,
    IMAGE_DIR,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    LoaderTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};

use super::question::get_questions;
use entity::{
    clauses::Entity as ClauseEntity,
    questions::QuestionWithAnswersModel,
    rule_question_answer::Entity as RuleQuestionAnswerEntity,
    rules::Model as RuleModel,
    systems::{
        ActiveModel as SystemActiveModel, Column as SystemColumn, Entity as SystemEntity,
        Model as SystemModel, NewSystemMultipartModel, SystemsWithPageCount, TestSystemModel,
        UpdateSystemModel, UpdateSystemMultipartModel,
    },
    users::{Column as UserColumn, Entity as UserEntity},
};
use std::collections::{HashMap, HashSet};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
    try_join,
};

pub async fn get_systems<C>(
    db: &C,
    params: SystemListPagination,
) -> Result<SystemsWithPageCount, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut query = SystemEntity::find();

    if let Some(_id) = params.user_id {
        query = query.filter(SystemColumn::UserId.eq(_id));
    }

    if !params.all_types.map_or(false, |all_types| all_types) {
        query = query.filter(SystemColumn::Private.eq(false));
    }

    if let Some(_name) = params.name {
        query = query.filter(SystemColumn::Name.like(format!("%{}%", _name)));
    }

    if let Some(_username) = params.username {
        query = query
            .inner_join(UserEntity)
            .filter(UserColumn::Username.like(format!("%{}%", _username)));
    }

    let raw_count = query.clone().count(db).await? as f64;

    let per_page = params.per_page.unwrap_or(20) as u64;
    let page = params.page.unwrap_or(1) as u64 - 1;

    let _systems = query
        .order_by_desc(SystemColumn::UpdatedAt)
        .limit(per_page)
        .offset(per_page * page)
        .all(db)
        .await?;

    Ok(SystemsWithPageCount {
        systems: _systems,
        pages: (raw_count / (per_page as f64)).ceil() as i64,
    })
}

pub async fn get_system<C>(db: &C, system_id: i32) -> Result<SystemModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(SystemEntity::find_by_id(system_id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Система не найдена".to_string()))?)
}

pub async fn get_ready_to_start_system<C>(db: &C, system_id: i32) -> Result<TestSystemModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let (_rules, _questions_with_answers, _objects) = try_join!(
        get_rules(db, system_id),
        get_questions(db, system_id),
        get_objects(db, system_id)
    )?;

    let _rules_with_question_rule: Vec<RuleModel> = _rules
        .as_slice()
        .into_iter()
        .filter_map(|rule| {
            if !rule.attribute_rule {
                return None;
            }
            Some(RuleModel {
                id: rule.id,
                system_id,
                attribute_rule: rule.attribute_rule,
            })
        })
        .collect();

    let clauses = _rules_with_question_rule
        .load_many(ClauseEntity, db)
        .await?;

    let _rules_with_question_deps: HashMap<i32, Vec<i32>> = clauses
        .into_iter()
        .zip(&_rules_with_question_rule)
        .map(|(clauses, rule)| {
            (
                rule.id,
                clauses
                    .into_iter()
                    .map(|clause| clause.question_id)
                    .collect(),
            )
        })
        .collect();

    let rule_question_answers = _rules_with_question_rule
        .load_many(RuleQuestionAnswerEntity, db)
        .await?;

    let mut rules_belonging_questions: HashMap<i32, HashSet<i32>> = HashMap::new();

    rule_question_answers
        .into_iter()
        .zip(&_rules_with_question_rule)
        .for_each(|(raw, rule)| {
            raw.into_iter().for_each(|raw_with_question_id| {
                let deps = _rules_with_question_deps
                    .get(&rule.id)
                    .cloned()
                    .unwrap_or_default();
                let dependancies: HashSet<i32> = deps.into_iter().collect();
                rules_belonging_questions
                    .entry(raw_with_question_id.question_id)
                    .and_modify(|dep| {
                        *dep = dep.union(&dependancies).cloned().collect();
                    })
                    .or_insert(dependancies.into_iter().collect());
            })
        });

    _questions_with_answers
        .as_slice()
        .into_iter()
        .for_each(|question| {
            rules_belonging_questions
                .entry(question.id)
                .or_insert(HashSet::new());
        });

    let belonging_questions_order = topological_sort(&rules_belonging_questions);

    let ordered_questions = belonging_questions_order
        .into_iter()
        .filter_map(|question_order_id| {
            _questions_with_answers
                .iter()
                .find(|question| question.id == question_order_id)
                .cloned()
        })
        .collect::<Vec<QuestionWithAnswersModel>>();

    Ok(TestSystemModel {
        questions: ordered_questions,
        rules: _rules,
        objects: _objects,
    })
}

pub async fn create_system<C>(
    db: &C,
    system_info: NewSystemMultipartModel,
    cookie_user_id: i32,
) -> Result<SystemModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let image_info = system_info.image;
    let image_name = image_info.as_ref().and_then(|info| {
        Some(format!(
            "{}_{}_{}",
            chrono::Utc::now().timestamp_millis(),
            info.metadata
                .name
                .clone()
                .map_or("image".to_string(), |name| name),
            info.metadata
                .file_name
                .clone()
                .map_or("unknown".to_string(), |file_name| file_name)
        ))
    });

    let new_system = SystemActiveModel {
        user_id: Set(cookie_user_id),
        about: Set(system_info.about),
        name: Set(system_info.name),
        image_uri: Set(image_name
            .as_ref()
            .and_then(|img_name| Some(format!("/images/{}", img_name)))),
        private: Set(system_info.private),
        ..Default::default()
    };

    let result = new_system.insert(db).await?;

    let _ = fs::create_dir_all(IMAGE_DIR).await;

    if let (Some(image_name), Some(image_info)) = (image_name, image_info) {
        let mut file = File::create(format!("{IMAGE_DIR}/{}", image_name))
            .await
            .or(Err(DbErr::Custom("Невозможно сохранить лого".to_string())))?;
        file.write(&image_info.contents)
            .await
            .or(Err(DbErr::Custom("Невозможно сохранить лого".to_string())))?;
    }

    Ok(result)
}

pub async fn update_system<C>(
    db: &C,
    system_id: i32,
    system_info: UpdateSystemMultipartModel,
) -> Result<SystemModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let image_info = system_info.image;

    let image_name = image_info.as_ref().and_then(|info| {
        Some(format!(
            "{}_{}_{}",
            chrono::Utc::now().timestamp_millis(),
            info.metadata
                .name
                .clone()
                .map_or("image".to_string(), |name| name),
            info.metadata
                .file_name
                .clone()
                .map_or("unknown".to_string(), |file_name| file_name),
        ))
    });

    let system_image_uri_old = SystemEntity::find_by_id(system_id)
        .one(db)
        .await?
        .and_then(|system| system.image_uri);

    let mut new_image_uri: Option<String>;

    if let Some(system_image_uri_old) = system_image_uri_old {
        if image_info.is_some() {
            let _ = fs::remove_file(format!(".{}", system_image_uri_old)).await;
        }
        new_image_uri = image_name
            .as_ref()
            .and_then(|img_name| Some(format!("/images/{}", img_name)));
        if let Some(is_image_removed) = system_info.is_image_removed {
            if is_image_removed {
                new_image_uri = Some("".to_string());
                let _ = fs::remove_file(format!(".{}", system_image_uri_old)).await;
            }
        }
    } else {
        new_image_uri = Some("".to_string());
    }

    let mut user_to_update = UpdateSystemModel {
        about: system_info.about,
        name: system_info.name,
        image_uri: new_image_uri,
        private: system_info.private,
    }
    .into_active_model();
    user_to_update.id = Set(system_id);
    let result = user_to_update.update(db).await?;

    if let (Some(img_info), Some(img_name)) = (image_info, image_name) {
        let mut file = File::create(format!("{IMAGE_DIR}/{}", img_name))
            .await
            .expect("Unable to create file");
        file.write(&img_info.contents)
            .await
            .expect("Error while create file");
    };

    Ok(result)
}

pub async fn delete_system<C>(db: &C, system_id: i32) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(SystemEntity::delete_by_id(system_id)
        .exec(db)
        .await?
        .rows_affected)
}
