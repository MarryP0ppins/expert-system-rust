use crate::{
    models::rule_attributevalue::{NewRuleAttributeValue, RuleAttributeValue},
    schema::rule_attributevalue::dsl::*,
};
use diesel::{delete, insert_into, prelude::*, result::Error};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_rule_attributevalues(
    connection: &mut AsyncPgConnection,
    rule_attribuevalue_info: Vec<NewRuleAttributeValue>,
) -> Result<(), Error> {
    match insert_into(rule_attributevalue)
        .values::<Vec<NewRuleAttributeValue>>(rule_attribuevalue_info)
        .get_result::<RuleAttributeValue>(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

pub async fn multiple_delete_rule_attributevalues(
    connection: &mut AsyncPgConnection,
    rule_attributevalues_ids: Vec<i32>,
) -> Result<(), Error> {
    match delete(rule_attributevalue.filter(id.eq_any(rule_attributevalues_ids)))
        .execute(connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
