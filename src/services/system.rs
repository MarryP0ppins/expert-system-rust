use crate::{
    models::system::{NewSystem, NewSystemMultipart, System, UpdateSystem, UpdateSystemMultipart},
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

    let system_image_uri_old: String;
    match systems
        .find(system_id)
        .select(image_uri)
        .first::<String>(connection)
        .await
    {
        Ok(result) => system_image_uri_old = result,
        Err(err) => return Err(err),
    }

    let result: System;
    match update(systems.find(system_id))
        .set::<UpdateSystem>(UpdateSystem {
            about: system_info.about,
            name: system_info.name,
            image_uri: image_name
                .as_ref()
                .and_then(|img_name| Some(format!("/images/{}", img_name))),
            private: system_info.private,
        })
        .get_result::<System>(connection)
        .await
    {
        Ok(ok) => result = ok,
        Err(err) => return Err(err),
    }

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
    match delete(systems.find(system_id)).execute(connection).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
