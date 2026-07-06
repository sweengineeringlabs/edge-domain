//! `Query` trait — a read operation that never mutates domain state.

use futures::future::BoxFuture;

use crate::api::query::QueryError;
use crate::api::query::types::{QueryExecuteRequest, QueryNameRequest, QueryNameResponse, QueryResultResponse};

/// A named read operation that returns data without mutating state.
pub trait Query: Send + Sync {
    /// The type returned on success.
    type Result: Send + 'static;

    /// Stable name identifying this query type.
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "query" })
    }

    /// Execute the query and return the result.
    fn execute(
        &self,
        req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<Self::Result>, QueryError>>;
}
