use crate::{
    config::Config,
    constants::COOKIE_NAME,
    models::email::Email,
    utils::{
        auth::{check_password, hash_password},
        generate_random_string::generate_random_string,
    },
};
use chrono::{Duration as ChronoDuration, Utc};
use entity::users::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity,
    ForgotPasswordModel, LoginUserModel, Model as UserModel, ResetPasswordModel, UpdateUserModel,
    UpdateUserResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Set, TransactionTrait, Unchanged,
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
        verified: None,
        verification_code: None,
        password_reset_at: None,
    };

    Ok(update_user.into_active_model().update(db).await?)
}

pub async fn create_user<C>(
    db: &C,
    user_info: UserModel,
    config: &Config,
) -> Result<UserModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = db.begin().await?;

    let verification_code = generate_random_string(20);

    let new_user = UserActiveModel {
        email: Set(user_info.email),
        password: Set(hash_password(&user_info.password)),
        username: Set(user_info.username),
        first_name: Set(user_info.first_name),
        last_name: Set(user_info.last_name),
        verification_code: Set(Some(verification_code.clone())),
        ..Default::default()
    };
    let user = new_user.insert(&txn).await?;

    let verification_url = format!(
        "{}/verifyemail/{}",
        config.frontend_origin, verification_code
    );

    let email_instance = Email::new(user.clone(), verification_url, config.clone());
    email_instance.send_verification_code().await.map_err(|_| {
        DbErr::Custom("Something bad happended while sending the verification code".to_string())
    })?;

    txn.commit().await?;

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

    if !user.verified {
        return Err(DbErr::Custom("Почта не подтверждена".to_string()));
    }

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

pub async fn verify_email<C>(
    db: &C,
    verification_code: String,
    cookie: Cookies,
    cookie_key: &Key,
) -> Result<UserModel, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = UserEntity::find()
        .filter(UserColumn::VerificationCode.eq(verification_code))
        .one(db)
        .await?
        .ok_or(DbErr::Custom(
            "Invalid verification code or user doesn't exist".to_string(),
        ))?;

    if user.verified {
        return Err(DbErr::Custom("User already verified".to_string()));
    }

    UserActiveModel {
        id: Unchanged(user.id),
        verified: Set(true),
        verification_code: Set(None),
        ..Default::default()
    }
    .update(db)
    .await?;

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

pub async fn forgot_password<C>(
    db: &C,
    forgot_password_model: ForgotPasswordModel,
    config: Config,
) -> Result<(), DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(forgot_password_model.email))
        .one(db)
        .await?
        .ok_or(DbErr::Custom(
            "Пользователь с указаной почтой не найден".to_string(),
        ))?;

    let password_reset_token = generate_random_string(20);
    let password_token_expires_in = 10; // 10 minutes
    let password_reset_at = Utc::now().naive_utc() + ChronoDuration::minutes(10);

    let password_reset_url = format!(
        "{}/resetpassword/{}",
        config.frontend_origin.to_owned(),
        password_reset_token
    );

    let email_instance = Email::new(user.clone(), password_reset_url, config.clone());
    email_instance
        .send_password_reset_token(password_token_expires_in)
        .await
        .map_err(|_| {
            DbErr::Custom(
                "Something bad happended while sending the password reset code".to_string(),
            )
        })?;

    UserActiveModel {
        id: Unchanged(user.id),
        verification_code: Set(Some(password_reset_token)),
        password_reset_at: Set(Some(password_reset_at)),
        ..Default::default()
    }
    .update(db)
    .await?;

    Ok(())
}

pub async fn reset_password<C>(
    db: &C,
    reset_password_model: ResetPasswordModel,
    reset_password_token: String,
) -> Result<(), DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = UserEntity::find()
        .filter(UserColumn::VerificationCode.eq(reset_password_token))
        .filter(UserColumn::PasswordResetAt.gt(Utc::now().naive_utc()))
        .one(db)
        .await?
        .ok_or(DbErr::Custom(
            "Токен востановления пароля недействителен".to_string(),
        ))?;

    UserActiveModel {
        id: Unchanged(user.id),
        verification_code: Set(None),
        password_reset_at: Set(None),
        password: Set(hash_password(&reset_password_model.password)),
        ..Default::default()
    }
    .update(db)
    .await?;

    Ok(())
}
