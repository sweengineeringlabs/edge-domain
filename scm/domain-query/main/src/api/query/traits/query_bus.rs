//! `QueryBus` trait — dispatches queries and returns their results.

use futures::future::BoxFuture;

use crate::api::query::QueryError;
use crate::api::query::types::{QueryDispatchRequest, QueryResultResponse};

/// Dispatches [`Query`](super::Query) instances and returns their results.
pub trait QueryBus: Send + Sync {
    /// The result type produced by queries dispatched through this bus.
    type Result: Send + 'static;

    /// Dispatch a query and return its result.
    fn dispatch(
        &self,
        req: QueryDispatchRequest<Self::Result>,
    ) -> BoxFuture<'_, Result<QueryResultResponse<Self::Result>, QueryError>>;
}
