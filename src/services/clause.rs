use crate::entity::clauses::{
    ActiveModel as ClauseActiveModel, Column as ClauseColumn, Entity as ClauseEntity,
    Model as ClauseModel, UpdateClauseModel,
};
use futures::future::try_join_all;
use sea_orm::*;

pub async fn get_clauses<C>(db: &C, rule_id: i32) -> Result<Vec<ClauseModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut clauses = ClauseEntity::find()
        .filter(ClauseColumn::RuleId.eq(rule_id))
        .order_by_asc(ClauseColumn::Id)
        .all(db)
        .await?;
    clauses.sort_by_key(|clause| clause.id);

    Ok(clauses)
}

pub async fn create_clauses<C>(
    db: &C,
    clause_info: Vec<ClauseModel>,
) -> Result<Vec<ClauseModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_clauses = clause_info.into_iter().map(|new_clause| {
        let model = ClauseActiveModel {
            rule_id: Set(new_clause.rule_id),
            compared_value: Set(new_clause.compared_value),
            logical_group: Set(new_clause.logical_group),
            operator: Set(new_clause.operator),
            question_id: Set(new_clause.question_id),
            ..Default::default()
        };
        model.insert(db)
    });

    let mut result = try_join_all(new_clauses).await?;

    result.sort_by_key(|clause| clause.id);

    Ok(result)
}

pub async fn multiple_delete_clauses<C>(db: &C, clauses_ids: Vec<i32>) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(ClauseEntity::delete_many()
        .filter(ClauseColumn::Id.is_in(clauses_ids))
        .exec(db)
        .await?
        .rows_affected)
}

pub async fn multiple_update_clauses<C>(
    db: &C,
    clauses_info: Vec<UpdateClauseModel>,
) -> Result<Vec<ClauseModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_clauses = clauses_info
        .into_iter()
        .map(|clause_for_update| clause_for_update.into_active_model().update(db));

    let mut result = try_join_all(new_clauses).await?;
    result.sort_by_key(|clause| clause.id);

    Ok(result)
}
