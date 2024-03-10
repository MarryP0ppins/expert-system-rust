use crate::{
    models::{
        answer as answer_model, attribute as attribute_model,
        attribute_value as attribute_value_model, clause as clause_model, error,
        history as history_model, object as object_model, question as question_model,
        response_body, rule as rule_model, rule_answer as rule_answer_model,
        rule_attributevalue as rule_attributevalue_model, system as system_model,
        user as user_model,
    },
    routes::{
        answer, attribute, attribute_value, clause, history, object, question, rule, rule_answer,
        rule_attributevalue, system, user,
    },
};
#[cfg(not(debug_assertions))]
use axum::Json;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
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
        rule_answer::rule_answer_create,
        rule_answer::rule_answer_multiple_delete,
        rule_attributevalue::rule_attributevalue_create,
        rule_attributevalue::rule_attributevalue_multiple_delete,
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
    ),
    components(schemas(
        error::CustomErrors,
        response_body::ResponseBodyAnswer,
        answer_model::Answer,
        answer_model::NewAnswer,
        answer_model::UpdateAnswer,
        attribute_value_model::AttributeValue,
        attribute_value_model::NewAttributeValue,
        attribute_value_model::UpdateAttributeValue,
        attribute_model::AttributeWithAttributeValues,
        attribute_model::NewAttributeWithAttributeValuesName,
        attribute_model::UpdateAttribute,
        clause_model::Clause,
        clause_model::NewClause,
        clause_model::UpdateClause,
        clause_model::RuleOperator,
        history_model::NewHistory,
        history_model::HistoryWithSystemAndUser,
        object_model::ObjectWithAttributesValues,
        object_model::UpdateObject,
        object_model::NewObjectWithAttributesValueIds,
        question_model::QuestionWithAnswers,
        question_model::NewQuestionWithAnswersBody,
        question_model::UpdateQuestion,
        rule_answer_model::NewRuleAnswer,
        rule_attributevalue_model::NewRuleAttributeValue,
        rule_model::NewRule,
        rule_model::RuleWithClausesAndEffects,
        system_model::System,
        system_model::NewSystemMultipart,
        system_model::UpdateSystemMultipart,
        system_model::SystemData,
        user_model::UserLogin,
        user_model::UserWithoutPassword,
        user_model::NewUser,
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
