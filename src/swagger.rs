use crate::{
    models::{
        answer as answer_model, attribute as attribute_model,
        attribute_value as attribute_value_model, clause as clause_model, error,
        history as history_model, system as system_model, user as user_model,
    },
    routes::{answer, attribute, attribute_value, clause, history, user},
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
        user::user_login,
        user::user_logout,
        user::user_registration,
        user::user_get,
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
        clause_model::Clause,
        clause_model::NewClause,
        clause_model::UpdateClause,
        clause_model::RuleOperator,
        history_model::NewHistory,
        history_model::HistoryWithSystemAndUser,
        system_model::System,
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
