use crate::{
    models::{
        attribute_value::AttributeValue,
        attribute_value_object::{AttributeValueObject, NewAttributeValueObject},
        object::{
            NewObject, NewObjectWithAttributesValueIds, Object, ObjectWithAttributesValues,
            UpdateObject,
        },
    },
    schema::{attributesvalue_object, attributesvalues, objects::dsl::*},
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

    let _grouped_attributes_values: Vec<Vec<(AttributeValueObject, AttributeValue)>>;
    match AttributeValueObject::belonging_to(&_object)
        .inner_join(attributesvalues::table)
        .select((
            attributesvalue_object::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(AttributeValueObject, AttributeValue)>(connection)
        .await
    {
        Ok(ok) => _grouped_attributes_values = ok.grouped_by(&_object),
        Err(_) => _grouped_attributes_values = vec![],
    };

    let result = _object
        .into_iter()
        .zip(_grouped_attributes_values)
        .map(
            |(_object, _attributes_values_objects)| ObjectWithAttributesValues {
                id: _object.id,
                system_id: _object.system_id,
                name: _object.name,
                attributes_values: _attributes_values_objects
                    .into_iter()
                    .map(|(_, attribute_values)| attribute_values)
                    .collect(),
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
                acc.0.push(raw.attributes_values_ids);
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

                insert_into(attributesvalue_object::table)
                    .values::<Vec<NewAttributeValueObject>>(
                        attributes_values_ids
                            .into_iter()
                            .zip(&_objects)
                            .flat_map(|(attributes_values, object)| {
                                attributes_values
                                    .into_iter()
                                    .map(|value| NewAttributeValueObject {
                                        attribute_value_id: value,
                                        object_id: object.id,
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

    let _attributes_values: Vec<Vec<(AttributeValueObject, AttributeValue)>>;
    match AttributeValueObject::belonging_to(&_objects)
        .inner_join(attributesvalues::table)
        .select((
            attributesvalue_object::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(AttributeValueObject, AttributeValue)>(connection)
        .await
    {
        Ok(ok) => _attributes_values = ok.grouped_by(&_objects),
        Err(_) => _attributes_values = vec![],
    };

    let result = _objects
        .into_iter()
        .zip(_attributes_values)
        .map(|(object, attribute_values)| ObjectWithAttributesValues {
            id: object.id,
            system_id: object.system_id,
            name: object.name,
            attributes_values: attribute_values
                .into_iter()
                .map(|(_, value)| value)
                .collect(),
        })
        .collect();

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

    let _grouped_attributes_values: Vec<Vec<(AttributeValueObject, AttributeValue)>>;
    match AttributeValueObject::belonging_to(&_objects)
        .inner_join(attributesvalues::table)
        .select((
            attributesvalue_object::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(AttributeValueObject, AttributeValue)>(connection)
        .await
    {
        Ok(ok) => _grouped_attributes_values = ok.grouped_by(&_objects),
        Err(_) => _grouped_attributes_values = vec![],
    };

    let result = _objects
        .into_iter()
        .zip(_grouped_attributes_values)
        .map(
            |(_object, _attributes_values_objects)| ObjectWithAttributesValues {
                id: _object.id,
                system_id: _object.system_id,
                name: _object.name,
                attributes_values: _attributes_values_objects
                    .into_iter()
                    .map(|(_, attribute_values)| attribute_values)
                    .collect(),
            },
        )
        .collect::<Vec<ObjectWithAttributesValues>>();

    Ok(result)
}
