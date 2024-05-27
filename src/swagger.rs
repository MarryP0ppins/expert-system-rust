use crate::{
    entity::{
        answers as answer_model, attributes as attributes_model,
        attributesvalues as attributesvalues_model, clauses as clause_model,
        histories as history_model,
        object_attribute_attributevalue as object_attribute_attributevalue_model,
        objects as object_model, questions as question_model,
        rule_attribute_attributevalue as rule_attribute_attributevalue_model,
        rule_question_answer as rule_question_answer_model, rules as rule_model,
        systems as system_model, users as user_model,
    },
    models::error,
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
        answer_model::Model,
        answer_model::UpdateAnswerModel,
        //attributesvalues_model::Model,
        attributesvalues_model::UpdateAttributeValueModel,
        attributes_model::UpdateAttributeModel,
        attributes_model::AttributeWithAttributeValuesModel,
        //object_attribute_attributevalue_model::Model,
        object_attribute_attributevalue_model::NewObjectAttributeAttributevalueWithoutObjectModel,
        //clause_model::Model,
        clause_model::UpdateClauseModel,
        clause_model::NewClauseWithoutRule,
        clause_model::NewClauseWithoutRule,
        //history_model::Model,
        history_model::HistoryWithSystem,
        object_model::ObjectWithAttributesValuesModel,
        object_model::UpdateObjectModel,
        object_model::NewObjectWithAttributesValueIdsModel,
        question_model::QuestionWithAnswersModel,
        question_model::NewQuestionWithAnswersModel,
        question_model::UpdateQuestionModel,
        //rule_question_answer_model::Model,
        rule_question_answer_model::NewRuleQuestionAnswerWithoutRuleModel,
        //rule_attribute_attributevalue_model::Model,
        rule_attribute_attributevalue_model::NewRuleAttributeAttributeValueWithoutRuleModel,
        rule_model::RuleWithClausesAndEffects,
        rule_model::NewRuleWithClausesAndEffects,
        //system_model::Model,
        system_model::NewSystemMultipartModel,
        system_model::UpdateSystemMultipartModel,
        system_model::SystemDeleteModel,
        //user_model::Model,
        user_model::LoginUserModel,
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
