use crate::entity::object_attribute_attributevalue::{
    ActiveModel as ObjectAttributeAttributeValueActiveModel,
    Column as ObjectAttributeAttributeValueColumn, Entity as ObjectAttributeAttributeValueEntity,
    Model as ObjectAttributeAttributeValueModel,
};

use futures::future::try_join_all;
use sea_orm::*;

pub async fn create_attribute_values_objects<C>(
    db: &C,
    attribute_values_objects: Vec<ObjectAttributeAttributeValueModel>,
) -> Result<Vec<ObjectAttributeAttributeValueModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_attribute_values_objects =
        attribute_values_objects
            .into_iter()
            .map(|new_attribute_values_object| {
                let model = ObjectAttributeAttributeValueActiveModel {
                    object_id: Set(new_attribute_values_object.object_id),
                    attribute_value_id: Set(new_attribute_values_object.attribute_value_id),
                    attribute_id: Set(new_attribute_values_object.attribute_id),
                    ..Default::default()
                };
                model.insert(db)
            });

    let mut result = try_join_all(new_attribute_values_objects).await?;
    result.sort_by_key(|attribute_values_object| attribute_values_object.id);

    Ok(result)
}

pub async fn multiple_delete_attribute_values_objects<C>(
    db: &C,
    attribute_values_objects_ids: Vec<i32>,
) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(ObjectAttributeAttributeValueEntity::delete_many()
        .filter(ObjectAttributeAttributeValueColumn::Id.is_in(attribute_values_objects_ids))
        .exec(db)
        .await?
        .rows_affected)
}
