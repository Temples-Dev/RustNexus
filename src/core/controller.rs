use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::Value;
use async_trait::async_trait;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// Define a generic filtering and sorting struct
#[derive(Deserialize)]
pub struct QueryParams {
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub filters: Option<Value>,
}

#[async_trait]
pub trait Controller< T, E = StatusCode>: Send + Sync
where
    E: From<StatusCode>,
    T: Send + 'static,
{
    // GET - Retrieve a resource by ID
    async fn get(_id: Path<String>) -> Result<Json<T>, E> {
        Err(StatusCode::NOT_IMPLEMENTED.into())
    }

    // GET - Retrieve all resources with pagination and query params
    async fn get_all(
        _pagination: Query<Pagination>,
        _params: Query<QueryParams>,
    ) -> Result<Json<Vec<T>>, E> {
        Err(StatusCode::NOT_IMPLEMENTED.into())
    }

    // POST - Create a new resource
    async fn create(_payload: Json<T>) -> Result<StatusCode, E> {
        Err(StatusCode::NOT_IMPLEMENTED.into())
    }

    // PUT - Update a resource (full update)
    async fn update(_id: Path<String>, _payload: Json<T>) -> Result<StatusCode, E> {
        Err(StatusCode::NOT_IMPLEMENTED.into())
    }

    // PATCH - Partial update of a resource
    async fn patch(_id: Path<String>, _payload: Json<serde_json::Value>) -> Result<StatusCode, E> {
        Err(StatusCode::NOT_IMPLEMENTED.into())
    }

    // DELETE - Remove a resource
    async fn delete(_id: Path<String>) -> Result<StatusCode, E> {
        Err(StatusCode::NOT_IMPLEMENTED.into())
    }
}
