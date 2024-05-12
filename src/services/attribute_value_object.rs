use crate::{
    models::attribute_value_object::{AttributeValueObject, NewAttributeValueObject},
    schema::attributesvalue_object::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_attribute_values_objects(
    connection: &mut AsyncPgConnection,
    attribute_values_objects: Vec<NewAttributeValueObject>,
) -> Result<(), Error> {
    match insert_into(attributesvalue_object)
        .values::<Vec<NewAttributeValueObject>>(attribute_values_objects)
        .get_results::<AttributeValueObject>(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

pub async fn multiple_delete_attribute_values_objects(
    connection: &mut AsyncPgConnection,
    attribute_values_objects_ids: Vec<i32>,
) -> Result<(), Error> {
    match delete(attributesvalue_object.filter(id.eq_any(attribute_values_objects_ids)))
        .execute(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
