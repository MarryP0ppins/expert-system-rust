use crate::{
    models::{
        error::CustomErrors, object_attribute_attributevalue::NewObjectAttributeAttributevalue,
    },
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

#[utoipa::path(
    post,
    path = "/object-attributevalue",
    context_path ="/api/v1",
    request_body = [NewObjectAttributeAttributevalue],
    responses(
        (status = 200, description = "AttributeValuesObjects and their dependences create successfully", body = CustomErrors, example = json!(())),
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
    Json(attribute_values_objects_info): Json<Vec<NewObjectAttributeAttributevalue>>,
) -> impl IntoResponse {
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match create_attribute_values_objects(&mut connection, attribute_values_objects_info).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
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
        (status = 200, description = "AttributeValuesObjects and their dependences deleted successfully", body = CustomErrors, example = json!(())),
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
    let mut connection = state
        .db_pool
        .get()
        .await
        .map_err(|err| CustomErrors::PoolConnectionError(err))?;

    match multiple_delete_attribute_values_objects(&mut connection, attribute_values_objects_info)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(CustomErrors::DieselError {
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
