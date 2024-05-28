use entity::rule_attribute_attributevalue::{
    ActiveModel as RuleAttributeAttributevalueActiveModel,
    Column as RuleAttributeAttributevalueColumn, Entity as RuleAttributeAttributevalueEntity,
    Model as RuleAttributeAttributevalueModel,
};
use futures::future::try_join_all;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

pub async fn create_rule_attribute_attributevalues<C>(
    db: &C,
    rule_attribuevalue_info: Vec<RuleAttributeAttributevalueModel>,
) -> Result<Vec<RuleAttributeAttributevalueModel>, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_rule_attribuevalues =
        rule_attribuevalue_info
            .into_iter()
            .map(|new_rule_attribuevalue| {
                let model = RuleAttributeAttributevalueActiveModel {
                    rule_id: Set(new_rule_attribuevalue.rule_id),
                    attribute_value_id: Set(new_rule_attribuevalue.attribute_value_id),
                    attribute_id: Set(new_rule_attribuevalue.attribute_id),
                    ..Default::default()
                };
                model.insert(db)
            });

    let mut result = try_join_all(new_rule_attribuevalues).await?;
    result.sort_by_key(|rule_attribuevalue| rule_attribuevalue.id);

    Ok(result)
}

pub async fn multiple_delete_rule_attribute_attributevalues<C>(
    db: &C,
    rule_attribute_attributevalues_ids: Vec<i32>,
) -> Result<u64, DbErr>
where
    C: ConnectionTrait + TransactionTrait,
{
    Ok(RuleAttributeAttributevalueEntity::delete_many()
        .filter(RuleAttributeAttributevalueColumn::Id.is_in(rule_attribute_attributevalues_ids))
        .exec(db)
        .await?
        .rows_affected)
}
