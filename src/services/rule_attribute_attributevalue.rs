use crate::{
    models::rule_attribute_attributevalue::NewRuleAttributeAttributeValue,
    schema::rule_attribute_attributevalue::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_rule_attribute_attributevalues(
    connection: &mut AsyncPgConnection,
    rule_attribuevalue_info: Vec<NewRuleAttributeAttributeValue>,
) -> Result<usize, Error> {
    Ok(insert_into(rule_attribute_attributevalue)
        .values::<Vec<NewRuleAttributeAttributeValue>>(rule_attribuevalue_info)
        .execute(connection)
        .await?)
}

pub async fn multiple_delete_rule_attribute_attributevalues(
    connection: &mut AsyncPgConnection,
    rule_attribute_attributevalues_ids: Vec<i32>,
) -> Result<usize, Error> {
    Ok(
        delete(rule_attribute_attributevalue.filter(id.eq_any(rule_attribute_attributevalues_ids)))
            .execute(connection)
            .await?,
    )
}
