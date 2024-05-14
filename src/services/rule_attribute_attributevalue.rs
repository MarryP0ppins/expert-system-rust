use crate::{
    models::rule_attribute_attributevalue::{
        NewRuleAttributeAttributeValue, RuleAttributeAttributeValue,
    },
    schema::rule_attribute_attributevalue::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_rule_attribute_attributevalues(
    connection: &mut AsyncPgConnection,
    rule_attribuevalue_info: Vec<NewRuleAttributeAttributeValue>,
) -> Result<(), Error> {
    match insert_into(rule_attribute_attributevalue)
        .values::<Vec<NewRuleAttributeAttributeValue>>(rule_attribuevalue_info)
        .get_result::<RuleAttributeAttributeValue>(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

pub async fn multiple_delete_rule_attribute_attributevalues(
    connection: &mut AsyncPgConnection,
    rule_attribute_attributevalues_ids: Vec<i32>,
) -> Result<(), Error> {
    match delete(
        rule_attribute_attributevalue.filter(id.eq_any(rule_attribute_attributevalues_ids)),
    )
    .execute(connection)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
