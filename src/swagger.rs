use crate::{
    models::{answer as answer_model, error},
    routes::answer,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        answer::answer_create,
        answer::answer_list,
        answer::answer_multiple_delete,
        answer::answer_multiple_update,
    ),
    components(schemas(
        error::DieselError,
        error::CustomErrors,
        answer_model::Answer,
        answer_model::NewAnswer,
        answer_model::UpdateAnswer
    ))
)]
pub struct ApiDoc;
