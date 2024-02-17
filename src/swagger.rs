use crate::{
    models::{answer as answer_model, error, user as user_model},
    routes::{answer, user},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::user_login,
        user::user_logout,
        user::user_registration,
        user::user_get,
        answer::answer_create,
        answer::answer_list,
        answer::answer_multiple_delete,
        answer::answer_multiple_update,
    ),
    components(schemas(
        error::CustomErrors,
        user_model::UserLogin,
        user_model::UserWithoutPassword,
        user_model::NewUser,
        answer_model::Answer,
        answer_model::NewAnswer,
        answer_model::UpdateAnswer
    ))
)]
pub struct ApiDoc;
