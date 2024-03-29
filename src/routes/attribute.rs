use crate::{
    models::{
        attribute::{NewAttributeWithAttributeValuesName, UpdateAttribute},
        error::CustomErrors,
        response_body::{ResponseBodyAttributes, ResponseBodyEmpty},
    },
    pagination::AttributeListPagination,
    services::attribute::{
        create_attributes, get_attributes, multiple_delete_attributes, multiple_update_attributes,
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
    path = "/attributes",
    context_path ="/api/v1",
    request_body = [NewAttributeWithAttributeValuesName],
    responses(
        (status = 200, description = "Attributes and their dependences create successfully", body = ResponseBodyAttributes),
        (status = 401, description = "Unauthorized to create Attributes and their dependences", body = ResponseBodyAttributes, example = json!(ResponseBodyAttributes::unauthorized_example()))
    )
)]
#[debug_handler]
pub async fn attribute_create(
    State(state): State<AppState>,
    Json(attribute_info): Json<Vec<NewAttributeWithAttributeValuesName>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyAttributes::from(CustomErrors::PoolConnectionError(err)),
    };

    match create_attributes(&mut connection, attribute_info).await {
        Ok(result) => ResponseBodyAttributes::from(result),
        Err(err) => ResponseBodyAttributes::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/attributes",
    context_path ="/api/v1",
    responses(
        (status = 200, description = "List matching Attributes and their dependences by query", body = ResponseBodyAttributes),
        (status = 401, description = "Unauthorized to list Attributes and their dependences", body = ResponseBodyAttributes, example = json!(ResponseBodyAttributes::unauthorized_example()))
    ),
    params(
        AttributeListPagination
    )
)]
#[debug_handler]
pub async fn attribute_list(
    State(state): State<AppState>,
    Query(pagination): Query<AttributeListPagination>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyAttributes::from(CustomErrors::PoolConnectionError(err)),
    };

    let pagination = pagination as AttributeListPagination;
    match get_attributes(&mut connection, pagination.system_id).await {
        Ok(result) => ResponseBodyAttributes::from(result),
        Err(err) => ResponseBodyAttributes::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/attributes/multiple_delete",
    context_path ="/api/v1",
    request_body = [i32],
    responses(
        (status = 200, description = "Attributes and their dependences deleted successfully", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty { succsess: true, data: None, error: None })),
        (status = 401, description = "Unauthorized to delete Attributes and their dependences", body = ResponseBodyEmpty, example = json!(ResponseBodyEmpty::unauthorized_example())),
        (status = 404, description = "Answers not found")
    )
)]
#[debug_handler]
pub async fn attribute_multiple_delete(
    State(state): State<AppState>,
    Json(attribute_info): Json<Vec<i32>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyEmpty::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_delete_attributes(&mut connection, attribute_info).await {
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
    path = "/attributes/multiple_update",
    context_path ="/api/v1",
    request_body = [UpdateAttribute],
    responses(
        (status = 200, description = "Attributes and their dependences updated successfully", body = ResponseBodyAttributes),
        (status = 401, description = "Unauthorized to update Attributes and their dependences", body = ResponseBodyAttributes, example = json!(ResponseBodyAttributes::unauthorized_example())),
        (status = 404, description = "Attributes and their dependences not found")
    )
)]
#[debug_handler]
pub async fn attribute_multiple_update(
    State(state): State<AppState>,
    Json(attribute_info): Json<Vec<UpdateAttribute>>,
) -> impl IntoResponse {
    let mut connection: PooledConnection<AsyncPgConnection>;
    match state.db_pool.get().await {
        Ok(ok) => connection = ok,
        Err(err) => return ResponseBodyAttributes::from(CustomErrors::PoolConnectionError(err)),
    };

    match multiple_update_attributes(&mut connection, attribute_info).await {
        Ok(result) => ResponseBodyAttributes::from(result),
        Err(err) => ResponseBodyAttributes::from(CustomErrors::DieselError {
            error: err,
            message: None,
        }),
    }
}

pub fn attribute_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attribute_create).get(attribute_list))
        .route("/multiple_delete", delete(attribute_multiple_delete))
        .route("/multiple_patch", patch(attribute_multiple_update))
}
