use crate::{
    constants::COOKIE_NAME,
    utils::auth::{check_password, hash_password},
};
use entity::users::{
    Column as UserColumn, Entity as UserEntity, LoginUserModel, Model as UserModel,
    UpdateUserModel, UpdateUserResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, TransactionTrait,
};
use tower_cookies::{
    cookie::{
        time::{Duration, OffsetDateTime},
        SameSite,
    },
    Cookie, Cookies, Key,
};

pub async fn get_user<C>(db: &C, user_id: i32) -> Result<UserModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(UserEntity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Пользователь не найден".to_string()))?)
}

pub async fn update_user<C>(
    db: &C,
    user_data: UpdateUserResponse,
    user_id: i32,
) -> Result<UserModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let update_user = UpdateUserModel {
        id: user_id,
        email: user_data.email,
        first_name: user_data.first_name,
        last_name: user_data.last_name,
        password: user_data.new_password,
    };

    Ok(update_user.into_active_model().update(db).await?)
}

pub async fn create_user<C>(
    db: &C,
    user_info: UserModel,
    cookie: Cookies,
    cookie_key: &Key,
) -> Result<UserModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_user = UserModel {
        password: hash_password(&user_info.password),
        ..user_info
    }
    .into_active_model();

    let user = new_user.insert(db).await?;

    cookie.private(cookie_key).add(
        Cookie::build((COOKIE_NAME, user.id.to_string()))
            .path("/")
            .secure(true)
            .http_only(false)
            .same_site(SameSite::Strict)
            .expires(OffsetDateTime::now_utc() + Duration::days(2))
            .into(),
    );

    Ok(user)
}

pub async fn login_user<C>(
    db: &C,
    user_info: LoginUserModel,
    cookie: Cookies,
    cookie_key: &Key,
) -> Result<UserModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(user_info.email))
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Пользователь не найден".to_string()))?;

    let null_cookie = Cookie::build((COOKIE_NAME, ""))
        .path("/")
        .expires(OffsetDateTime::now_utc());
    cookie.private(cookie_key).add(null_cookie.into());

    check_password(&user_info.password, &user.password).or(Err(DbErr::Custom(
        "Предоставлены неверные учетные данные".to_string(),
    )))?;

    cookie.private(cookie_key).add(
        Cookie::build((COOKIE_NAME, user.id.to_string()))
            .path("/")
            .secure(true)
            .http_only(false)
            .same_site(SameSite::Strict)
            .expires(OffsetDateTime::now_utc() + Duration::days(2))
            .into(),
    );

    Ok(user)
}
