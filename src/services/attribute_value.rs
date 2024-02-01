use crate::{
    models::attribute_value::{AttributeValue, NewAttributeValue, UpdateAttributeValue},
    schema::attributesvalues::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};

pub fn get_attribute_values(
    connection: &mut PgConnection,
    attribute: i32,
) -> Result<Vec<AttributeValue>, Error> {
    match attributesvalues
        .filter(attribute_id.eq(attribute))
        .load::<AttributeValue>(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn create_attributes_values(
    connection: &mut PgConnection,
    attributes_values_info: Vec<NewAttributeValue>,
) -> Result<Vec<AttributeValue>, Error> {
    match insert_into(attributesvalues)
        .values::<Vec<NewAttributeValue>>(attributes_values_info)
        .get_results::<AttributeValue>(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_delete_attributes_values(
    connection: &mut PgConnection,
    attributes_values_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(attributesvalues.filter(id.eq_any(attributes_values_ids))).execute(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_update_attributes_values(
    connection: &mut PgConnection,
    attributes_values_info: Vec<UpdateAttributeValue>,
) -> Result<Vec<AttributeValue>, Error> {
    match attributes_values_info
        .iter()
        .map(|attribute_value_raw| {
            update(attributesvalues.find(attribute_value_raw.id))
                .set::<UpdateAttributeValue>(attribute_value_raw.clone())
                .get_result::<AttributeValue>(connection)
        })
        .collect()
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
