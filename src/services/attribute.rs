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
    let new_attributes = db
        .transaction::<_, Vec<AttributeWithAttributeValuesModel>, DbErr>(|txn| {
            Box::pin(async move {
                let new_attribute = attribute_info.into_iter().map(|attribute_raw| async move {
                    let new_attribute = AttributeActiveModel {
                        system_id: Set(attribute_raw.system_id),
                        name: Set(attribute_raw.name),
                        ..Default::default()
                    };
                    let created_attribute = new_attribute.insert(txn).await;
                    match created_attribute {
                        Ok(result) => {
                            let values_to_create = attribute_raw
                                .values_name
                                .into_iter()
                                .map(|value_name| AttributeValueModel {
                                    id: -1,
                                    attribute_id: result.id,
                                    value: value_name,
                                })
                                .collect();
                            let values = create_attributes_values(txn, values_to_create).await?;
                            Ok(AttributeWithAttributeValuesModel {
                                id: result.id,
                                system_id: result.system_id,
                                name: result.name,
                                values,
                            })
                        }
                        Err(err) => Err(err),
                    }
                });

                let mut result = try_join_all(new_attribute).await?;
                result.sort_by(|a, b| a.id.cmp(&b.id));

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

    // let txn = db.begin().await?;

    // let new_attribute = attribute_info.into_iter().map(|attribute_raw| async move {
    //     let new_attribute = AttributeActiveModel {
    //         system_id: Set(attribute_raw.system_id),
    //         name: Set(attribute_raw.name),
    //         ..Default::default()
    //     };
    //     let created_attribute = new_attribute.insert(txn).await;
    //     match created_attribute {
    //         Ok(result) => {
    //             let values_to_create = attribute_raw
    //                 .values_name
    //                 .into_iter()
    //                 .map(|value_name| AttributeValueModel {
    //                     id: -1,
    //                     attribute_id: result.id,
    //                     value: value_name,
    //                 })
    //                 .collect();
    //             let values = create_attributes_values(&txn, values_to_create).await?;
    //             Ok(AttributeWithAttributeValuesModel {
    //                 id: result.id,
    //                 system_id: result.system_id,
    //                 name: result.name,
    //                 values,
    //             })
    //         }
    //         Err(err) => Err(err),
    //     }
    // });

    // let mut new_attributes = try_join_all(new_attribute).await?;
    // new_attributes.sort_by(|a, b| a.id.cmp(&b.id));

    // txn.commit().await?;

    Ok(new_attributes)
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
