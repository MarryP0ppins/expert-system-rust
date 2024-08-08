use entity::likes::{
    ActiveModel as LikesActiveModel, Column as LikesColumn, Entity as LikesEntity,
    Model as LikesModel,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr,
    EntityTrait, ModelTrait, QueryFilter, Set, Statement, TransactionTrait,
};

pub async fn get_likes<C>(db: &C, user_id: i32) -> Result<Vec<LikesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(LikesEntity::find()
        .filter(LikesColumn::UserId.eq(user_id))
        .all(db)
        .await?)
}

pub async fn create_like<C>(db: &C, like_info: LikesModel) -> Result<LikesModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let model = LikesActiveModel {
        id: NotSet,
        user_id: Set(like_info.user_id),
        system_id: Set(like_info.system_id),
        ..Default::default()
    };
    let result = model.insert(db).await?;

    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "UPDATE \"public\".\"systems\" SET stars = stars + 1 WHERE id = $1;",
        [like_info.system_id.into()],
    ))
    .await?;

    Ok(result)
}

pub async fn delete_like<C>(db: &C, like_id: i32) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let like = LikesEntity::find()
        .filter(LikesColumn::Id.eq(like_id))
        .one(db)
        .await?;

    if let Some(like_model) = like {
        like_model.clone().delete(db).await?;

        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "UPDATE \"public\".\"systems\" SET stars = stars - 1 WHERE id = $1;",
            [like_model.system_id.into()],
        ))
        .await?;
    }

    Ok(like_id as u64)
}
