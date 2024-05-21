use crate::{
    models::{
        answer as answer_model, attribute as attribute_model,
        attribute_value as attribute_value_model, clause as clause_model, error,
        history as history_model, object as object_model,
        object_attribute_attributevalue as object_attribute_attributevalue_model,
        question as question_model, rule as rule_model,
        rule_attribute_attributevalue as rule_attribute_attributevalue_model,
        rule_question_answer as rule_question_answer_model, system as system_model,
        user as user_model,
    },
    routes::{
        answer, attribute, attribute_value, clause, history, object,
        object_attribute_attributevalue, question, rule, rule_attribute_attributevalue,
        rule_question_answer, system, user,
    },
};
#[cfg(not(debug_assertions))]
use axum::Json;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        answer::answer_create,
        answer::answer_list,
        answer::answer_multiple_delete,
        answer::answer_multiple_update,
        attribute_value::attribute_value_create,
        attribute_value::attribute_value_list,
        attribute_value::attribute_value_multiple_delete,
        attribute_value::attribute_value_multiple_update,
        attribute::attribute_create,
        attribute::attribute_list,
        attribute::attribute_multiple_delete,
        attribute::attribute_multiple_update,
        object_attribute_attributevalue::attribute_values_objects_create,
        object_attribute_attributevalue::attribute_values_objects_multiple_delete,
        clause::clause_create,
        clause::clause_list,
        clause::clause_multiple_delete,
        clause::clause_multiple_update,
        history::history_create,
        history::history_list,
        history::history_delete,
        object::object_create,
        object::object_list,
        object::object_multiple_delete,
        object::object_multiple_update,
        question::question_create,
        question::question_list,
        question::question_multiple_delete,
        question::question_multiple_update,
        rule_question_answer::rule_question_answer_create,
        rule_question_answer::rule_question_answer_multiple_delete,
        rule_attribute_attributevalue::rule_attribute_attributevalue_create,
        rule_attribute_attributevalue::rule_attribute_attributevalue_multiple_delete,
        rule::rule_create,
        rule::rule_list,
        rule::rule_multiple_delete,
        system::system_create,
        system::system_list,
        system::system_retrieve,
        system::system_start,
        system::system_partial_update,
        system::system_delete,
        user::user_login,
        user::user_logout,
        user::user_registration,
        user::user_get,
        user::user_patch
    ),
    components(schemas(
        error::CustomErrors,
        answer_model::Answer,
        answer_model::NewAnswer,
        answer_model::UpdateAnswer,
        attribute_value_model::AttributeValue,
        attribute_value_model::NewAttributeValue,
        attribute_value_model::UpdateAttributeValue,
        attribute_model::AttributeWithAttributeValues,
        attribute_model::NewAttributeWithAttributeValuesName,
        attribute_model::UpdateAttribute,
        object_attribute_attributevalue_model::ObjectAttributeAttributevalue,
        object_attribute_attributevalue_model::NewObjectAttributeAttributevalue,
        object_attribute_attributevalue_model::NewObjectAttributeAttributevalueWithoutObject,
        clause_model::Clause,
        clause_model::NewClause,
        clause_model::UpdateClause,
        clause_model::RuleOperator,
        clause_model::NewClauseWithoutRule,
        history_model::NewHistory,
        history_model::HistoryWithSystem,
        object_model::ObjectWithAttributesValues,
        object_model::UpdateObject,
        object_model::NewObjectWithAttributesValueIds,
        question_model::QuestionWithAnswers,
        question_model::NewQuestionWithAnswersBody,
        question_model::UpdateQuestion,
        rule_question_answer_model::RuleQuestionAnswer,
        rule_question_answer_model::NewRuleQuestionAnswer,
        rule_question_answer_model::NewRuleQuestionAnswerWithoutRule,
        rule_attribute_attributevalue_model::RuleAttributeAttributeValue,
        rule_attribute_attributevalue_model::NewRuleAttributeAttributeValue,
        rule_attribute_attributevalue_model::NewRuleAttributeAttributeValueWithoutRule,
        rule_model::NewRule,
        rule_model::RuleWithClausesAndEffects,
        rule_model::NewRuleWithClausesAndEffects,
        system_model::System,
        system_model::NewSystemMultipart,
        system_model::UpdateSystemMultipart,
        system_model::SystemData,
        system_model::SystemDelete,
        system_model::SystemsWithPageCount,
        user_model::UserLogin,
        user_model::UserWithoutPassword,
        user_model::NewUser,
        user_model::UpdateUserResponse
    ))
)]
pub struct ApiDoc;

#[cfg(not(debug_assertions))]
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("session_id"))),
            )
        }
    }
}
