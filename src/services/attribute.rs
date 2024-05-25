use std::sync::Arc;

use futures::future::try_join_all;
use sea_orm::*;

use crate::entity::attributes::{
    ActiveModel as AttributeActiveModel, AttributeWithAttributeValuesModel,
    Column as AttributeColumn, Entity as AttributeEntity, NewAttributeWithAttributeValuesModel,
    UpdateAttributeModel,
};
use crate::entity::attributesvalues::{
    Entity as AttributeValueEntity, Model as AttributeValueModel,
};

use super::attribute_value::create_attributes_values;

pub async fn get_attributes<C>(
    db: &C,
    system_id: i32,
) -> Result<Vec<AttributeWithAttributeValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let attribute_with_attributevalues = AttributeEntity::find()
        .filter(AttributeColumn::SystemId.eq(system_id))
        .find_with_related(AttributeValueEntity)
        .all(db)
        .await?;

    let result = attribute_with_attributevalues
        .into_iter()
        .map(
            |(attribute, attribute_values)| AttributeWithAttributeValuesModel {
                id: attribute.id,
                system_id: attribute.system_id,
                name: attribute.name,
                values: attribute_values,
            },
        )
        .collect::<Vec<AttributeWithAttributeValuesModel>>();

    Ok(result)
}

pub async fn create_attributes<C>(
    db: &C,
    attribute_info: Vec<NewAttributeWithAttributeValuesModel>,
) -> Result<Vec<AttributeWithAttributeValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = db.begin().await?;

    let shared_txn = Arc::new(&txn);
    let new_attributes = attribute_info.into_iter().map(|attribute_raw| {
        let txn_cloned = shared_txn.clone();
        async move {
            let new_attribute = AttributeActiveModel {
                system_id: Set(attribute_raw.system_id),
                name: Set(attribute_raw.name),
                ..Default::default()
            };
            let created_attribute = new_attribute.insert(*txn_cloned).await?;
            let values_to_create = attribute_raw
                .values_name
                .into_iter()
                .map(|value_name| AttributeValueModel {
                    id: -1,
                    attribute_id: created_attribute.id,
                    value: value_name,
                })
                .collect();
            let values = create_attributes_values(*txn_cloned, values_to_create).await?;
            Ok::<AttributeWithAttributeValuesModel, DbErr>(AttributeWithAttributeValuesModel {
                id: created_attribute.id,
                system_id: created_attribute.system_id,
                name: created_attribute.name,
                values,
            })
        }
    });

    let mut result = try_join_all(new_attributes).await?;
    result.sort_by(|a, b| a.id.cmp(&b.id));

    txn.commit().await?;

    Ok(result)
}

pub async fn multiple_delete_attributes<C>(db: &C, attributes_ids: Vec<i32>) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(AttributeEntity::delete_many()
        .filter(AttributeColumn::Id.is_in(attributes_ids))
        .exec(db)
        .await?
        .rows_affected)
}

pub async fn multiple_update_attributes<C>(
    db: &C,
    attributes_info: Vec<UpdateAttributeModel>,
) -> Result<Vec<AttributeWithAttributeValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let updated_attributes = attributes_info
        .into_iter()
        .map(|attributes_for_update| async move {
            attributes_for_update.into_active_model().update(db).await
        });

    let mut attributes = try_join_all(updated_attributes).await?;
    attributes.sort_by(|a, b| a.id.cmp(&b.id));

    let attributes_values = attributes.load_many(AttributeValueEntity, db).await?;

    let result = attributes
        .into_iter()
        .zip(attributes_values)
        .map(|(attribute, attribute_values)| {
            let mut values = attribute_values;
            values.sort_by(|a, b| a.id.cmp(&b.id));
            AttributeWithAttributeValuesModel {
                id: attribute.id,
                system_id: attribute.system_id,
                name: attribute.name,
                values,
            }
        })
        .collect();

    Ok(result)
}
