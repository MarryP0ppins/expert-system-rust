use crate::{
    models::attribute_value::{AttributeValue, NewAttributeValue, UpdateAttributeValue},
    schema::attributesvalues::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_attribute_values(
    connection: &mut AsyncPgConnection,
    attribute: i32,
) -> Result<Vec<AttributeValue>, Error> {
    match attributesvalues
        .filter(attribute_id.eq(attribute))
        .load::<AttributeValue>(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn create_attributes_values(
    connection: &mut AsyncPgConnection,
    attributes_values_info: Vec<NewAttributeValue>,
) -> Result<Vec<AttributeValue>, Error> {
    match insert_into(attributesvalues)
        .values::<Vec<NewAttributeValue>>(attributes_values_info)
        .get_results::<AttributeValue>(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn multiple_delete_attributes_values(
    connection: &mut AsyncPgConnection,
    attributes_values_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(attributesvalues.filter(id.eq_any(attributes_values_ids)))
        .execute(connection)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn multiple_update_attributes_values(
    connection: &mut AsyncPgConnection,
    attributes_values_info: Vec<UpdateAttributeValue>,
) -> Result<Vec<AttributeValue>, Error> {
    let mut attributes_values: Vec<AttributeValue> = vec![];

    for attribute_value_raw in attributes_values_info.into_iter() {
        match update(attributesvalues.find(attribute_value_raw.id))
            .set::<UpdateAttributeValue>(attribute_value_raw)
            .get_result::<AttributeValue>(connection)
            .await
        {
            Ok(result) => attributes_values.push(result),
            Err(err) => return Err(err),
        }
    }

    Ok(attributes_values)
}
