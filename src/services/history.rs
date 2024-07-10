use chrono::Local;
use entity::{
    histories::{Entity as HistoryEntity, HistoryWithSystem, Model as HistoryModel},
    systems::{Column as SystemColumn, Entity as SystemEntity},
    users::{Column as UserColumn, Entity as UserEntity},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, TransactionTrait,
};

pub async fn get_histories<C>(
    db: &C,
    _system: Option<i32>,
    _user: Option<i32>,
) -> Result<Vec<HistoryWithSystem>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut query = HistoryEntity::find();

    if let Some(param) = _system {
        query = query
            .inner_join(SystemEntity)
            .filter(SystemColumn::Id.eq(param));
    }
    if let Some(param) = _user {
        query = query
            .inner_join(UserEntity)
            .filter(UserColumn::Id.eq(param));
    }

    let histories = query.find_also_related(SystemEntity).all(db).await?;

    let mut result = histories
        .into_iter()
        .map(|(history, system_option)| {
            let system = system_option.ok_or(DbErr::Custom("system error".to_string()))?;
            Ok(HistoryWithSystem {
                id: history.id,
                system,
                answered_questions: history.answered_questions,
                results: history.results,
                started_at: history.started_at,
                finished_at: history.finished_at,
            })
        })
        .collect::<Result<Vec<HistoryWithSystem>, DbErr>>()?;

    result.sort_by_key(|history| -1 * history.id);

    Ok(result)
}

pub async fn create_history<C>(
    db: &C,
    history_info: HistoryModel,
) -> Result<HistoryWithSystem, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut new_history_active_model = history_info.into_active_model();
    new_history_active_model.id = NotSet;
    let new_history = new_history_active_model.insert(db).await?;

    let system = SystemEntity::find_by_id(new_history.system_id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Ошибка создания записи истории".to_string()))?;

    let result = HistoryWithSystem {
        id: new_history.id,
        system,
        answered_questions: new_history.answered_questions,
        results: new_history.results,
        started_at: Local::now().naive_local(),
        finished_at: Local::now().naive_local(),
    };

    Ok(result)
}

pub async fn delete_history<C>(db: &C, history_id: i32) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(HistoryEntity::delete_by_id(history_id)
        .exec(db)
        .await?
        .rows_affected)
}
