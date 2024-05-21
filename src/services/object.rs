use crate::{
    models::{
        object::{
            NewObject, NewObjectWithAttributesValueIds, Object, ObjectWithAttributesValues,
            UpdateObject,
        },
        object_attribute_attributevalue::{
            NewObjectAttributeAttributevalue, ObjectAttributeAttributevalue,
        },
    },
    schema::{object_attribute_attributevalue, objects::dsl::*},
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};

pub async fn get_objects(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<ObjectWithAttributesValues>, Error> {
    let _object = objects
        .filter(system_id.eq(system))
        .load::<Object>(connection)
        .await?;

    let _attributes_values: Vec<ObjectAttributeAttributevalue> =
        ObjectAttributeAttributevalue::belonging_to(&_object)
            .load::<ObjectAttributeAttributevalue>(connection)
            .await?;

    let result = _attributes_values
        .grouped_by(&_object)
        .into_iter()
        .zip(_object)
        .map(
            |(_object_attribute_attributevalue, _object)| ObjectWithAttributesValues {
                id: _object.id,
                system_id: _object.system_id,
                name: _object.name,
                object_attribute_attributevalue_ids: _object_attribute_attributevalue,
            },
        )
        .collect::<Vec<ObjectWithAttributesValues>>();

    Ok(result)
}

pub async fn create_objects(
    connection: &mut AsyncPgConnection,
    object_info: Vec<NewObjectWithAttributesValueIds>,
) -> Result<Vec<ObjectWithAttributesValues>, Error> {
    let (attributes_values_ids, new_objects) =
        object_info
            .into_iter()
            .fold((vec![], vec![]), |mut acc, raw| {
                acc.0.push(raw.object_attribute_attributevalue_ids);
                acc.1.push(NewObject {
                    system_id: raw.system_id,
                    name: raw.name,
                });
                acc
            });

    let mut _objects: Vec<Object> = vec![];

    match connection
        .transaction(|connection| {
            async {
                _objects = insert_into(objects)
                    .values::<Vec<NewObject>>(new_objects)
                    .get_results::<Object>(connection)
                    .await?;

                insert_into(object_attribute_attributevalue::table)
                    .values::<Vec<NewObjectAttributeAttributevalue>>(
                        attributes_values_ids
                            .into_iter()
                            .zip(&_objects)
                            .flat_map(|(attributes_values, object)| {
                                attributes_values.into_iter().map(|value| {
                                    NewObjectAttributeAttributevalue {
                                        attribute_value_id: value.attribute_value_id,
                                        object_id: object.id,
                                        attribute_id: value.attribute_id,
                                    }
                                })
                            })
                            .collect(),
                    )
                    .execute(connection)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    let _attributes_values: Vec<ObjectAttributeAttributevalue> =
        ObjectAttributeAttributevalue::belonging_to(&_objects)
            .load::<ObjectAttributeAttributevalue>(connection)
            .await?;

    let result = _attributes_values
        .grouped_by(&_objects)
        .into_iter()
        .zip(_objects)
        .map(
            |(_object_attribute_attributevalue, _object)| ObjectWithAttributesValues {
                id: _object.id,
                system_id: _object.system_id,
                name: _object.name,
                object_attribute_attributevalue_ids: _object_attribute_attributevalue,
            },
        )
        .collect::<Vec<ObjectWithAttributesValues>>();

    Ok(result)
}

pub async fn multiple_delete_objects(
    connection: &mut AsyncPgConnection,
    objects_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(objects.filter(id.eq_any(objects_ids)))
        .execute(connection)
        .await?)
}

pub async fn multiple_update_objects(
    connection: &mut AsyncPgConnection,
    object_info: Vec<UpdateObject>,
) -> Result<Vec<ObjectWithAttributesValues>, Error> {
    let mut _objects: Vec<Object> = vec![];

    for object_raw in object_info.into_iter() {
        match update(objects.find(object_raw.id))
            .set::<UpdateObject>(object_raw)
            .get_result::<Object>(connection)
            .await
        {
            Ok(result) => _objects.push(result),
            Err(err) => return Err(err),
        }
    }

    let _attributes_values: Vec<ObjectAttributeAttributevalue> =
        ObjectAttributeAttributevalue::belonging_to(&_objects)
            .load::<ObjectAttributeAttributevalue>(connection)
            .await?;

    let result = _attributes_values
        .grouped_by(&_objects)
        .into_iter()
        .zip(_objects)
        .map(
            |(_object_attribute_attributevalue, _object)| ObjectWithAttributesValues {
                id: _object.id,
                system_id: _object.system_id,
                name: _object.name,
                object_attribute_attributevalue_ids: _object_attribute_attributevalue,
            },
        )
        .collect::<Vec<ObjectWithAttributesValues>>();

    Ok(result)
}
