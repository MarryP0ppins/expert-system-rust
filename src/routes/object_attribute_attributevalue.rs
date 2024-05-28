use crate::{
    error::CustomErrors,
    services::object_attribute_attributevalue::{
        create_attribute_values_objects, multiple_delete_attribute_values_objects,
    },
    AppState,
};

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use entity::object_attribute_attributevalue::ObjectAttributeAttributeValueModel;

#[utoipa::path(
    post,
    path = "/object-attributevalue",
    context_path ="/api/v1",
    request_body = [ObjectAttributeAttributeValueModel],
    responses(
        (status = 200, description = "AttributeValuesObjects and their dependences create successfully", body = [ObjectAttributeAttributeValueModel]),
        (status = 401, description = "Unauthorized to create AttributeValuesObjects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_values_objects_create(
    State(state): State<AppState>,
    Json(attribute_values_objects_info): Json<Vec<ObjectAttributeAttributeValueModel>>,
) -> impl IntoResponse {
    match create_attribute_values_objects(&state.db_sea, attribute_values_objects_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/object-attributevalue/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "AttributeValuesObjects and their dependences deleted successfully", body = u64),
        (status = 401, description = "Unauthorized to delete AttributeValuesObjects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        })),
        (status = 404, description = "AttributeValuesObjects not found")
    ),
    security(("Cookie" = []))
)]
#[debug_handler]
pub async fn attribute_values_objects_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_values_objects_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    match multiple_delete_attribute_values_objects(&state.db_sea, attribute_values_objects_info)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(CustomErrors::SeaORMError {
            error: err,
            message: None,
        }),
    }
}

pub fn object_attribute_attributevalue_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_values_objects_create))
        .route(
            "/multiple_delete",
            delete(attribute_values_objects_multiple_delete),
        )
}
