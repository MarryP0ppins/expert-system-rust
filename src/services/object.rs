use std::sync::Arc;

use entity::{
    object_attribute_attributevalue::{
        Entity as ObjectAttributeAttributeValueEntity, Model as ObjectAttributeAttributeValueModel,
    },
    objects::{
        ActiveModel as ObjectActiveModel, Column as ObjectColumn, Entity as ObjectEntity,
        NewObjectWithAttributesValueIdsModel, ObjectWithAttributesValuesModel, UpdateObjectModel,
    },
};
use futures::future::try_join_all;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    LoaderTrait, QueryFilter, Set, TransactionTrait,
};

use super::object_attribute_attributevalue::create_attribute_values_objects;

pub async fn get_objects<C>(
    db: &C,
    system_id: i32,
) -> Result<Vec<ObjectWithAttributesValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let objects = ObjectEntity::find()
        .filter(ObjectColumn::SystemId.eq(system_id))
        .find_with_related(ObjectAttributeAttributeValueEntity)
        .all(db)
        .await?;

    let mut result = objects
        .into_iter()
        .map(|(object, mut objects_ids)| {
            objects_ids.sort_by_key(|objects_id| objects_id.id);
            ObjectWithAttributesValuesModel {
                id: object.id,
                system_id: object.system_id,
                name: object.name,
                object_attribute_attributevalue_ids: objects_ids,
            }
        })
        .collect::<Vec<ObjectWithAttributesValuesModel>>();
    result.sort_by_key(|obj| obj.id);

    Ok(result)
}

pub async fn create_objects<C>(
    db: &C,
    object_info: Vec<NewObjectWithAttributesValueIdsModel>,
) -> Result<Vec<ObjectWithAttributesValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = db.begin().await?;
    let shared_txn = Arc::new(&txn);

    let new_objects = object_info.into_iter().map(|object_raw| {
        let txn_cloned = shared_txn.clone();
        async move {
            let new_object = ObjectActiveModel {
                system_id: Set(object_raw.system_id),
                name: Set(object_raw.name),
                ..Default::default()
            };
            let created_object = new_object.insert(*txn_cloned).await?;
            let values_to_create = object_raw
                .object_attribute_attributevalue_ids
                .into_iter()
                .map(|ids| ObjectAttributeAttributeValueModel {
                    id: -1,
                    object_id: created_object.id,
                    attribute_id: ids.attribute_id,
                    attribute_value_id: ids.attribute_value_id,
                })
                .collect();
            let ids = create_attribute_values_objects(*txn_cloned, values_to_create).await?;
            Ok::<ObjectWithAttributesValuesModel, DbErr>(ObjectWithAttributesValuesModel {
                id: created_object.id,
                system_id: created_object.system_id,
                name: created_object.name,
                object_attribute_attributevalue_ids: ids,
            })
        }
    });

    let mut result = try_join_all(new_objects).await?;
    result.sort_by_key(|obj| obj.id);

    txn.commit().await?;

    Ok(result)
}

pub async fn multiple_delete_objects<C>(db: &C, objects_ids: Vec<i32>) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(ObjectEntity::delete_many()
        .filter(ObjectColumn::Id.is_in(objects_ids))
        .exec(db)
        .await?
        .rows_affected)
}

pub async fn multiple_update_objects<C>(
    db: &C,
    object_info: Vec<UpdateObjectModel>,
) -> Result<Vec<ObjectWithAttributesValuesModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let updated_objects = object_info
        .into_iter()
        .map(|objects_for_update| objects_for_update.into_active_model().update(db));

    let mut objects = try_join_all(updated_objects).await?;
    objects.sort_by_key(|obj| obj.id);

    let objects_values = objects
        .load_many(ObjectAttributeAttributeValueEntity, db)
        .await?;

    let result = objects
        .into_iter()
        .zip(objects_values)
        .map(|(object, mut object_ids)| {
            object_ids.sort_by_key(|objects_id| objects_id.id);
            ObjectWithAttributesValuesModel {
                id: object.id,
                system_id: object.system_id,
                name: object.name,
                object_attribute_attributevalue_ids: object_ids,
            }
        })
        .collect();

    Ok(result)
}
