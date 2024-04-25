use std::collections::{HashMap, HashSet};

use crate::{
    models::{
        clause::Clause,
        question::QuestionWithAnswers,
        rule::Rule,
        rule_answer::RuleAnswer,
        system::{
            NewSystem, NewSystemMultipart, System, SystemData, SystemsWithPageCount, UpdateSystem,
            UpdateSystemMultipart,
        },
    },
    pagination::SystemListPagination,
    schema::{rule_answer, rules, systems, systems::dsl::*, users},
    utils::topological_sort::topological_sort,
    IMAGE_DIR,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use super::{question::get_questions, rule::get_rules};

pub async fn get_systems(
    connection: &mut AsyncPgConnection,
    params: SystemListPagination,
) -> Result<SystemsWithPageCount, Error> {
    let mut query = systems.inner_join(users::table).into_boxed();
    let mut raw_count_query = systems.inner_join(users::table).into_boxed();

    if let Some(_name) = params.name {
        query = query.filter(name.like(format!("%{}%", _name)));
        raw_count_query = raw_count_query.filter(name.like(format!("%{}%", _name)));
    }

    if let Some(_username) = params.username {
        query = query.filter(users::username.like(format!("%{}%", _username)));
        raw_count_query = raw_count_query.filter(users::username.like(format!("%{}%", _username)));
    }

    if let Some(_id) = params.user_id {
        query = query.filter(users::id.eq(_id));
        raw_count_query = raw_count_query.filter(users::id.eq(_id));
    }

    let raw_count = raw_count_query
        .count()
        .get_result::<i64>(connection)
        .await? as f64;

    let _systems = query
        .select(systems::all_columns)
        .limit(params.per_page.unwrap_or(20).into())
        .offset((params.per_page.unwrap_or(20) * (params.page.unwrap_or(1) - 1)).into())
        .load::<System>(connection)
        .await?;

    Ok(SystemsWithPageCount {
        systems: _systems,
        pages: (raw_count / (params.per_page.unwrap_or(20) as f64)).ceil() as i64,
    })
}

pub async fn get_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<System, Error> {
    Ok(systems.find(system_id).first::<System>(connection).await?)
}

pub async fn get_ready_to_start_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<SystemData, Error> {
    let _questions_with_answers = get_questions(connection, system_id).await?;

    let _rules_with_question_rule = rules::table
        .filter(rules::system_id.eq(system_id))
        .load::<Rule>(connection)
        .await?
        .into_iter()
        .filter(|rule| !rule.attribute_rule)
        .collect::<Vec<Rule>>();

    let mut _rules_with_question_deps: HashMap<i32, Vec<i32>> = HashMap::new();
    match Clause::belonging_to(&_rules_with_question_rule)
        .load::<Clause>(connection)
        .await
    {
        Ok(ok) => (ok as Vec<Clause>)
            .grouped_by(&_rules_with_question_rule)
            .into_iter()
            .zip(&_rules_with_question_rule)
            .for_each(|(clauses, rule)| {
                _rules_with_question_deps.insert(
                    rule.id,
                    clauses
                        .as_slice()
                        .into_iter()
                        .map(|clause| clause.question_id)
                        .collect(),
                );
            }),
        Err(err) => return Err(err),
    };

    let mut rules_belonging_questions: HashMap<i32, Vec<i32>> = HashMap::new();
    match RuleAnswer::belonging_to(&_rules_with_question_rule)
        .select(rule_answer::all_columns)
        .load::<RuleAnswer>(connection)
        .await
    {
        Ok(ok) => {
            (ok as Vec<RuleAnswer>)
                .grouped_by(&_rules_with_question_rule)
                .into_iter()
                .zip(&_rules_with_question_rule)
                .for_each(|(raw, rule)| {
                    raw.into_iter().for_each(|raw_with_question_id| {
                        let dependancies: HashSet<i32> = HashSet::from_iter(
                            _rules_with_question_deps
                                .get(&rule.id)
                                .unwrap_or(&vec![])
                                .to_vec()
                                .into_iter(),
                        );
                        rules_belonging_questions
                            .entry(raw_with_question_id.question_id)
                            .and_modify(|dep| {
                                *dep = HashSet::from_iter(dep.clone().into_iter())
                                    .union(&dependancies)
                                    .cloned()
                                    .collect();
                            })
                            .or_insert(dependancies.into_iter().collect());
                    })
                });
        }
        Err(err) => return Err(err),
    };

    let rules_with_clauses_and_effects = get_rules(connection, system_id).await?;
    let belonging_questions_order = topological_sort(&rules_belonging_questions);

    let questions_order = (HashSet::from_iter(
        rules_with_clauses_and_effects
            .as_slice()
            .into_iter()
            .map(|rule| rule.id),
    ) as HashSet<i32>)
        .difference(&HashSet::from_iter(
            belonging_questions_order.clone().into_iter(),
        ))
        .cloned()
        .chain(belonging_questions_order.into_iter())
        .collect::<Vec<i32>>();

    let ordered_questions: Vec<QuestionWithAnswers> = questions_order
        .into_iter()
        .filter_map(|question_order_id| {
            _questions_with_answers
                .as_slice()
                .into_iter()
                .find(|&question| question.id == question_order_id)
                .and_then(|borrow| Some(borrow.clone()))
        })
        .collect();

    Ok(SystemData {
        questions: ordered_questions,
        rules: rules_with_clauses_and_effects,
    })
}

pub async fn create_system(
    connection: &mut AsyncPgConnection,
    system_info: NewSystemMultipart,
) -> Result<System, Error> {
    let image_info = system_info.image;
    let image_name = format!(
        "{}_{}_{}",
        chrono::Utc::now().timestamp_millis(),
        image_info.metadata.name.clone().expect("No name"),
        image_info.metadata.file_name.clone().expect("No file name")
    );

    let result = insert_into(systems)
        .values::<NewSystem>(NewSystem {
            user_id: system_info.user_id,
            about: system_info.about,
            name: system_info.name,
            image_uri: format!("/images/{}", image_name),
            private: system_info.private,
        })
        .get_result::<System>(connection)
        .await?;

    let _ = fs::create_dir_all(IMAGE_DIR).await;

    let mut file = File::create(format!("{IMAGE_DIR}/{}", image_name))
        .await
        .expect("Unable to create file");
    file.write(&image_info.contents)
        .await
        .expect("Error while create file");

    Ok(result)
}

pub async fn update_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
    system_info: UpdateSystemMultipart,
) -> Result<System, Error> {
    let image_info = system_info.image;

    let image_name = image_info.as_ref().and_then(|info| {
        Some(format!(
            "{}_{}_{}",
            chrono::Utc::now().timestamp_millis(),
            info.metadata.name.clone().expect("No name"),
            info.metadata.file_name.clone().expect("No file name")
        ))
    });

    let system_image_uri_old = systems
        .find(system_id)
        .select(image_uri)
        .first::<String>(connection)
        .await?;

    let result = update(systems.find(system_id))
        .set::<UpdateSystem>(UpdateSystem {
            about: system_info.about,
            name: system_info.name,
            image_uri: image_name
                .as_ref()
                .and_then(|img_name| Some(format!("/images/{}", img_name))),
            private: system_info.private,
        })
        .get_result::<System>(connection)
        .await?;

    if let (Some(img_info), Some(img_name)) = (image_info, image_name) {
        let _ = fs::remove_file(format!(".{}", system_image_uri_old)).await;

        let mut file = File::create(format!("{IMAGE_DIR}/{}", img_name))
            .await
            .expect("Unable to create file");
        file.write(&img_info.contents)
            .await
            .expect("Error while create file");
    };

    Ok(result)
}

pub async fn delete_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<usize, Error> {
    Ok(delete(systems.find(system_id)).execute(connection).await?)
}
