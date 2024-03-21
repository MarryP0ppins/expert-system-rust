use crate::{
    models::{
        attribute::{
            Attribute, AttributeWithAttributeValues, NewAttribute,
            NewAttributeWithAttributeValuesName, UpdateAttribute,
        },
        attribute_value::{AttributeValue, NewAttributeValue},
    },
    schema::{attributes::dsl::*, attributesvalues},
};
use diesel::{delete, insert_into, prelude::*, result::Error, update};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};

pub async fn get_attributes(
    connection: &mut AsyncPgConnection,
    system: i32,
) -> Result<Vec<AttributeWithAttributeValues>, Error> {
    let _attributes = attributes
        .filter(system_id.eq(system))
        .load::<Attribute>(connection)
        .await?;

    let _attributes_values: Vec<AttributeValue>;
    match AttributeValue::belonging_to(&_attributes)
        .load::<AttributeValue>(connection)
        .await
    {
        Ok(ok) => _attributes_values = ok,
        Err(_) => _attributes_values = vec![],
    };

    let result = _attributes_values
        .grouped_by(&_attributes)
        .into_iter()
        .zip(_attributes)
        .map(
            |(attribute_values, attribute)| AttributeWithAttributeValues {
                id: attribute.id,
                system_id: attribute.system_id,
                name: attribute.name,
                values: attribute_values,
            },
        )
        .collect::<Vec<AttributeWithAttributeValues>>();

    Ok(result)
}

pub async fn create_attributes(
    connection: &mut AsyncPgConnection,
    attribute_info: Vec<NewAttributeWithAttributeValuesName>,
) -> Result<Vec<AttributeWithAttributeValues>, Error> {
    let (attributes_values_bodies, attributes_raws) =
        attribute_info
            .into_iter()
            .fold((vec![], vec![]), |mut acc, raw| {
                acc.0.push(raw.values_name);
                acc.1.push(NewAttribute {
                    system_id: raw.system_id,
                    name: raw.name,
                });
                acc
            });

    let mut new_attributes: Vec<Attribute> = vec![];
    let mut attributes_values: Vec<Vec<AttributeValue>> = vec![];

    match connection
        .transaction(|connection| {
            async {
                new_attributes = insert_into(attributes)
                    .values::<Vec<NewAttribute>>(attributes_raws)
                    .get_results::<Attribute>(connection)
                    .await?;

                attributes_values = insert_into(attributesvalues::table)
                    .values::<Vec<NewAttributeValue>>(
                        attributes_values_bodies
                            .into_iter()
                            .zip(&new_attributes)
                            .flat_map(|(attribute_value_bodies, attribute)| {
                                attribute_value_bodies
                                    .into_iter()
                                    .map(|value| NewAttributeValue {
                                        attribute_id: attribute.id,
                                        value,
                                    })
                            })
                            .collect(),
                    )
                    .get_results::<AttributeValue>(connection)
                    .await?
                    .grouped_by(&new_attributes);

                Ok(())
            }
            .scope_boxed()
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    let result = new_attributes
        .into_iter()
        .zip(attributes_values)
        .map(
            |(attribute, attribute_values)| AttributeWithAttributeValues {
                id: attribute.id,
                system_id: attribute.system_id,
                name: attribute.name,
                values: attribute_values,
            },
        )
        .collect();

    Ok(result)
}

pub async fn multiple_delete_attributes(
    connection: &mut AsyncPgConnection,
    attributes_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(delete(attributes.filter(id.eq_any(attributes_ids)))
        .execute(connection)
        .await?)
}

pub async fn multiple_update_attributes(
    connection: &mut AsyncPgConnection,
    attributes_info: Vec<UpdateAttribute>,
) -> Result<Vec<AttributeWithAttributeValues>, Error> {
    let mut _attributes: Vec<Attribute> = vec![];

    for attribute_raw in attributes_info.into_iter() {
        match update(attributes.find(attribute_raw.id))
            .set::<UpdateAttribute>(attribute_raw)
            .get_result::<Attribute>(connection)
            .await
        {
            Ok(result) => _attributes.push(result),
            Err(err) => return Err(err),
        }
    }

    let _attributes_values: Vec<AttributeValue>;
    match AttributeValue::belonging_to(&_attributes)
        .load::<AttributeValue>(connection)
        .await
    {
        Ok(ok) => _attributes_values = ok,
        Err(_) => _attributes_values = vec![],
    };

    let result = _attributes_values
        .grouped_by(&_attributes)
        .into_iter()
        .zip(_attributes)
        .map(
            |(attribute_values, attribute)| AttributeWithAttributeValues {
                id: attribute.id,

                system_id: attribute.system_id,
                name: attribute.name,
                values: attribute_values,
            },
        )
        .collect::<Vec<AttributeWithAttributeValues>>();

    Ok(result)
}
