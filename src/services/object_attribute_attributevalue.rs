use crate::{
    models::object_attribute_attributevalue::{
        NewObjectAttributeAttributevalue, ObjectAttributeAttributevalue,
    },
    schema::object_attribute_attributevalue::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_attribute_values_objects(
    connection: &mut AsyncPgConnection,
    attribute_values_objects: Vec<NewObjectAttributeAttributevalue>,
) -> Result<(), Error> {
    match insert_into(object_attribute_attributevalue)
        .values::<Vec<NewObjectAttributeAttributevalue>>(attribute_values_objects)
        .get_results::<ObjectAttributeAttributevalue>(connection)
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
    match delete(object_attribute_attributevalue.filter(id.eq_any(attribute_values_objects_ids)))
        .execute(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
