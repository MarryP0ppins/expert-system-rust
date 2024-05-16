use std::collections::{HashMap, HashSet};

use crate::{
    models::{
        clause::Clause,
        question::QuestionWithAnswers,
        rule::Rule,
        rule_question_answer::RuleQuestionAnswer,
        system::{
            NewSystem, NewSystemMultipart, System, SystemData, SystemsWithPageCount, UpdateSystem,
            UpdateSystemMultipart,
        },
    },
    pagination::SystemListPagination,
    schema::{
        rule_question_answer, rules,
        systems::{self, dsl::*},
        users,
    },
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

    if let Some(_id) = params.user_id {
        query = query.filter(users::id.eq(_id));
        raw_count_query = raw_count_query.filter(users::id.eq(_id));
    }

    if let Some(all_types) = params.all_types {
        if !all_types {
            query = query.filter(private.eq(false));
            raw_count_query = raw_count_query.filter(private.eq(false));
        }
    } else {
        query = query.filter(private.eq(false));
        raw_count_query = raw_count_query.filter(private.eq(false));
    }

    if let Some(_name) = params.name {
        query = query.filter(name.like(format!("%{}%", _name)));
        raw_count_query = raw_count_query.filter(name.like(format!("%{}%", _name)));
    }

    if let Some(_username) = params.username {
        query = query.filter(users::username.like(format!("%{}%", _username)));
        raw_count_query = raw_count_query.filter(users::username.like(format!("%{}%", _username)));
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
    //println!("111111111111111111111111\n{:?}", &_questions_with_answers);
    let _rules_with_question_rule = rules::table
        .filter(rules::system_id.eq(system_id))
        .load::<Rule>(connection)
        .await?
        .into_iter()
        .filter(|rule| !rule.attribute_rule)
        .collect::<Vec<Rule>>();

    // println!(
    //     "2222222222222222222222222\n{:?}",
    //     &_rules_with_question_rule
    // );
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
    //println!("3333333333333333333333\n{:?}", &_rules_with_question_deps);
    let mut rules_belonging_questions: HashMap<i32, Vec<i32>> = HashMap::new();
    match RuleQuestionAnswer::belonging_to(&_rules_with_question_rule)
        .select(rule_question_answer::all_columns)
        .load::<RuleQuestionAnswer>(connection)
        .await
    {
        Ok(ok) => {
            (ok as Vec<RuleQuestionAnswer>)
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

    _questions_with_answers
        .as_slice()
        .into_iter()
        .for_each(|question| {
            rules_belonging_questions
                .entry(question.id)
                .or_insert(vec![]);
        });

    // println!(
    //     "44444444444444444444444444\n{:?}",
    //     &rules_belonging_questions
    // );
    let rules_with_clauses_and_effects = get_rules(connection, system_id).await?;
    let belonging_questions_order = topological_sort(&rules_belonging_questions);

    // println!(
    //     "55555555555555555555555555\n{:?}",
    //     &belonging_questions_order
    // );

    let ordered_questions: Vec<QuestionWithAnswers> = belonging_questions_order
        .into_iter()
        .filter_map(|question_order_id| {
            _questions_with_answers
                .as_slice()
                .into_iter()
                .find(|&question| question.id == question_order_id)
                .and_then(|borrow| Some(borrow.clone()))
        })
        .collect();
    //println!("7777777777777777777777777777\n{:?}", &ordered_questions);
    Ok(SystemData {
        questions: ordered_questions,
        rules: rules_with_clauses_and_effects,
    })
}

pub async fn create_system(
    connection: &mut AsyncPgConnection,
    system_info: NewSystemMultipart,
    cookie_user_id: i32,
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

    let result = insert_into(systems)
        .values::<NewSystem>(NewSystem {
            user_id: cookie_user_id,
            about: system_info.about,
            name: system_info.name,
            image_uri: image_name
                .as_ref()
                .and_then(|img_name| Some(format!("/images/{}", img_name))),
            private: system_info.private,
        })
        .get_result::<System>(connection)
        .await?;

    let _ = fs::create_dir_all(IMAGE_DIR).await;

    if let (Some(image_name), Some(image_info)) = (image_name, image_info) {
        let mut file = File::create(format!("{IMAGE_DIR}/{}", image_name))
            .await
            .expect("Unable to create file");
        file.write(&image_info.contents)
            .await
            .expect("Error while create file");
    }

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
        .first::<Option<String>>(connection)
        .await?;

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

    let result = update(systems.find(system_id))
        .set::<UpdateSystem>(UpdateSystem {
            about: system_info.about,
            name: system_info.name,
            image_uri: new_image_uri,
            private: system_info.private,
        })
        .get_result::<System>(connection)
        .await?;

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

pub async fn delete_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<usize, Error> {
    Ok(delete(systems.find(system_id)).execute(connection).await?)
}
