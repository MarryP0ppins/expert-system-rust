use crate::{
    models::{attribute_value_object::NewAttributeValueObject, error::CustomErrors},
    services::attribute_value_object::{
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
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/object-attributevalue",
    context_path ="/api/v1",
    request_body = [NewAttributeValueObject],
    responses(
        (status = 200, description = "AttributeValuesObjects and their dependences create successfully", body = CustomErrors, example = json!(())),
        (status = 401, description = "Unauthorized to create AttributeValuesObjects and their dependences", body = CustomErrors, example = json!(CustomErrors::StringError {
            status: StatusCode::UNAUTHORIZED,
            error: "Not authorized".to_string(),
        }))
    )
)]
#[debug_handler]
pub async fn attribute_values_objects_create(
    State(state): State<AppState>,
    Json(attribute_values_objects_info): Json<Vec<NewAttributeValueObject>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

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
    )
)]
#[debug_handler]
pub async fn attribute_values_objects_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_values_objects_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return Err(CustomErrors::PoolConnectionError(err)),
    };

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

pub fn object_attributevalue_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_values_objects_create))
        .route(
            "/multiple_delete",
            delete(attribute_values_objects_multiple_delete),
        )
}
