use crate::entity::attributesvalues::{
    ActiveModel as AttributeValueActiveModel, Column as AttributeValueColumn,
    Entity as AttributeValueEntity, Model as AttributeValueModel, UpdateAttributeValueModel,
};
use futures::future::try_join_all;
use sea_orm::*;

pub async fn get_attribute_values<C>(
    db: &C,
    attribute_id: i32,
) -> Result<Vec<AttributeValueModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(AttributeValueEntity::find()
        .filter(AttributeValueColumn::AttributeId.eq(attribute_id))
        .order_by_asc(AttributeValueColumn::Id)
        .all(db)
        .await?)
}

pub async fn create_attributes_values<C>(
    db: &C,
    attributes_values_info: Vec<AttributeValueModel>,
) -> Result<Vec<AttributeValueModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_attributevalues = attributes_values_info
        .into_iter()
        .map(|new_attributevalue| {
            let model = AttributeValueActiveModel {
                attribute_id: Set(new_attributevalue.attribute_id),
                value: Set(new_attributevalue.value),
                ..Default::default()
            };
            model.insert(db)
        });

    let mut result = try_join_all(new_attributevalues).await?;

    result.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(result)
}

pub async fn multiple_delete_attributes_values<C>(
    db: &C,
    attributes_values_ids: Vec<i32>,
) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(AttributeValueEntity::delete_many()
        .filter(AttributeValueColumn::Id.is_in(attributes_values_ids))
        .exec(db)
        .await?
        .rows_affected)
}

pub async fn multiple_update_attributes_values<C>(
    db: &C,
    attributes_values_info: Vec<UpdateAttributeValueModel>,
) -> Result<Vec<AttributeValueModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_attributes_values =
        attributes_values_info
            .into_iter()
            .map(|attributes_values_for_update| {
                attributes_values_for_update.into_active_model().update(db)
            });

    let mut result = try_join_all(new_attributes_values).await?;

    result.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(result)
}
