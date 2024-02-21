use crate::{
    models::system::{NewSystem, NewSystemMultipart, System, UpdateSystem},
    schema::systems::dsl::*,
    IMAGE_DIR,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub async fn get_systems(
    connection: &mut AsyncPgConnection,
    _name: Option<&str>,
    _user_id: Option<i32>,
) -> Result<Vec<System>, Error> {
    let mut query = systems.into_boxed();

    if let Some(param) = _name {
        query = query.filter(name.like(format!("%{}%", param)));
    }

    if let Some(_user_id) = _user_id {
        query = query.filter(user_id.eq(_user_id));
    }

    match query.load::<System>(connection).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn get_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<System, Error> {
    match systems.find(system_id).first::<System>(connection).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
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

    let result: System;
    match insert_into(systems)
        .values::<NewSystem>(NewSystem {
            user_id: system_info.user_id,
            about: system_info.about,
            name: system_info.name,
            image_uri: format!("/images/{}", image_name),
            private: system_info.private,
        })
        .get_result::<System>(connection)
        .await
    {
        Ok(ok) => result = ok,
        Err(err) => return Err(err),
    }

    let _ = fs::create_dir_all(IMAGE_DIR).await;

    let mut file = File::create(format!("{IMAGE_DIR}/tokio_{}", image_name))
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
    system_info: UpdateSystem,
) -> Result<System, Error> {
    match update(systems.find(system_id))
        .set::<UpdateSystem>(system_info)
        .get_result::<System>(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn delete_system(
    connection: &mut AsyncPgConnection,
    system_id: i32,
) -> Result<usize, Error> {
    match delete(systems.find(system_id)).execute(connection).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
