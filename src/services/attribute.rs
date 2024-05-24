use futures::future::try_join_all;
use sea_orm::*;

use crate::entity::attributes::{
    ActiveModel as AttributeActiveModel, AttributeWithAttributeValuesModel,
    Column as AttributeColumn, Entity as AttributeEntity, Model as AttributeModel,
    UpdateAttributeModel,
};
use crate::entity::attributesvalues::Entity as AttributeValueEntity;

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
    attribute_info: Vec<AttributeWithAttributeValuesModel>,
) -> Result<Vec<AttributeWithAttributeValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let (attributes_values_bodies, attributes_raws) =
        attribute_info
            .into_iter()
            .fold((vec![], vec![]), |mut acc, raw| {
                acc.0.extend_from_slice(&raw.values);
                acc.1.push(AttributeActiveModel {
                    system_id: Set(raw.system_id),
                    name: Set(raw.name),
                    ..Default::default()
                });
                acc
            });

    let new_attributes = db
        .transaction::<_, Vec<AttributeModel>, DbErr>(|txn| {
            Box::pin(async move {
                let new_attribute = attributes_raws
                    .into_iter()
                    .map(|new_attribute| async move { new_attribute.insert(txn).await });

                let mut result = try_join_all(new_attribute).await?;
                result.sort_by(|a, b| a.id.cmp(&b.id));
                create_attributes_values(txn, attributes_values_bodies).await?;

                Ok(result)
            })
        })
        .await
        .or_else(|err| {
            Err(DbErr::Custom(format!(
                "Transaction error: {}",
                err.to_string()
            )))
        })?;

    let new_attributes_values = new_attributes.load_many(AttributeValueEntity, db).await?;

    let result = new_attributes
        .into_iter()
        .zip(new_attributes_values)
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
            let mut model = AttributeEntity::find_by_id(attributes_for_update.id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find attribute".to_owned()))?
                .into_active_model();
            model.name = Set(attributes_for_update.name);
            model.update(db).await
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
