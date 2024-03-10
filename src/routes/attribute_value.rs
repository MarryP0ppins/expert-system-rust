use crate::{
    models::{
        attribute_value::{NewAttributeValue, UpdateAttributeValue},
        error::CustomErrors,
        response_body::{ResponseBodyAttributeValues, ResponseBodyEmpty},
    },
    pagination::AttributeValueListPagination,
    services::attribute_value::{
        create_attributes_values, get_attribute_values, multiple_delete_attributes_values,
        multiple_update_attributes_values,
    },
    AppState,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

#[utoipa::path(
    post,
    path = "/attributevalues",
    context_path ="/api/v1",
    request_body = [NewAttributeValue],
    responses(
        (status = 200, description = "AttributeValues create successfully", body = ResponseBodyAttributeValues),
        (status = 401, description = "Unauthorized to create AttributeValues", body = ResponseBodyAttributeValues, example = json!(ResponseBodyAttributeValues::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn attribute_value_create(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<NewAttributeValue>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return ResponseBodyAttributeValues::from(CustomErrors::PoolConnectionError(err))
        }
    };

    match create_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => ResponseBodyAttributeValues::from(result),
        Err(err) => ResponseBodyAttributeValues::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/attributevalues",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching AttributeValues by query", body = ResponseBodyAttributeValues),
        (status = 401, description = "Unauthorized to list AttributeValues", body = ResponseBodyAttributeValues, example = json!(ResponseBodyAttributeValues::unauthorized_example()))
    ),
    params(
        AttributeValueListPagination
    )
)]
#[debug_handler]
pub async fn attribute_value_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeValueListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return ResponseBodyAttributeValues::from(CustomErrors::PoolConnectionError(err))
        }
    };

    let pagination = pagination as AttributeValueListPagination;
    match get_attribute_values(&mut connection, pagination.attribute_id).await {
        Ok(result) => ResponseBodyAttributeValues::from(result),
        Err(err) => ResponseBodyAttributeValues::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/attributevalues/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "AttributeValues deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete AttributeValues", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "AttributeValues not found")
    )
)]
#[debug_handler]
pub async fn attribute_value_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_attributes_values(&mut connection, attribute_value_info).await {
        Ok(_) => ResponseBodyEmpty {
            succsess: true,
            data: None,
            error: None,
        },
        Err(err) => ResponseBodyEmpty::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    patch,
    path = "/attributevalues/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAttributeValue],
    responses(
        (status = 200, description = "AttributeValues updated successfully", body = ResponseBodyAttributeValues),
        (status = 401, description = "Unauthorized to update AttributeValues", body = ResponseBodyAttributeValues, example = json!(ResponseBodyAttributeValues::unauthorized_example())),
        (status = 404, description = "AttributeValues not found")
    )
)]
pub async fn attribute_value_multiple_update(
    State(state): State<AppState>,
    Json(attribute_value_info): Json<Vec<UpdateAttributeValue>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => {
            return ResponseBodyAttributeValues::from(CustomErrors::PoolConnectionError(err))
        }
    };

    match multiple_update_attributes_values(&mut connection, attribute_value_info).await {
        Ok(result) => ResponseBodyAttributeValues::from(result),
        Err(err) => ResponseBodyAttributeValues::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn attribute_value_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_value_create).get(attribute_value_list))
        .route("/multiple_delete", delete(attribute_value_multiple_delete))
        .route("/multiple_patch", patch(attribute_value_multiple_update))
}
