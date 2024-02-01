use crate::{
    models::{
        attribute_value::AttributeValue,
        attribute_value_object::AttributeValueObject,
        object::{
            NewObject, NewObjectWithAttributesValueIds, Object, ObjectWithAttributesValues,
            UpdateObject,
        },
    },
    schema::{attributesvalue_object, attributesvalues, objects::dsl::*},
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};

pub fn get_objects(
    connection: &mut PgConnection,
    system: i32,
) -> Result<Vec<ObjectWithAttributesValues>, Error> {
    let _object: Vec<Object>;
    match objects
        .filter(system_id.eq(system))
        .load::<Object>(connection)
    {
        Ok(ok) => _object = ok,
        Err(err) => return Err(err),
    };

    let _grouped_attributes_values: Vec<Vec<(AttributeValueObject, AttributeValue)>>;
    match AttributeValueObject::belonging_to(&_object)
        .inner_join(attributesvalues::table)
        .select((
            attributesvalue_object::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(AttributeValueObject, AttributeValue)>(connection)
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

pub fn create_object(
    connection: &mut PgConnection,
    object_info: NewObjectWithAttributesValueIds,
) -> Result<ObjectWithAttributesValues, Error> {
    let attributes_values_ids = object_info.attributes_values_ids;

    let _object: Object;
    match insert_into(objects)
        .values::<NewObject>(NewObject {
            system_id: object_info.system_id,
            name: object_info.name,
        })
        .get_result::<Object>(connection)
    {
        Ok(ok) => _object = ok,
        Err(err) => return Err(err),
    };

    let _attributes_values: Vec<AttributeValue>;
    match attributesvalues::table
        .filter(attributesvalues::id.eq_any(attributes_values_ids))
        .load::<AttributeValue>(connection)
    {
        Ok(ok) => _attributes_values = ok,
        Err(err) => return Err(err),
    };

    let result = ObjectWithAttributesValues {
        id: _object.id,
        system_id: _object.system_id,
        name: _object.name,
        attributes_values: _attributes_values,
    };

    Ok(result)
}

pub fn multiple_delete_objects(
    connection: &mut PgConnection,
    objects_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(objects.filter(id.eq_any(objects_ids))).execute(connection) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub fn multiple_update_objects(
    connection: &mut PgConnection,
    object_info: Vec<UpdateObject>,
) -> Result<Vec<ObjectWithAttributesValues>, Error> {
    let _objects: Vec<Object>;

    match object_info
        .iter()
        .map(|object_raw| {
            update(objects.find(object_raw.id))
                .set::<UpdateObject>(object_raw.clone())
                .get_result::<Object>(connection)
        })
        .collect()
    {
        Ok(result) => _objects = result,
        Err(err) => return Err(err),
    }

    let _grouped_attributes_values: Vec<Vec<(AttributeValueObject, AttributeValue)>>;
    match AttributeValueObject::belonging_to(&_objects)
        .inner_join(attributesvalues::table)
        .select((
            attributesvalue_object::all_columns,
            attributesvalues::all_columns,
        ))
        .load::<(AttributeValueObject, AttributeValue)>(connection)
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
