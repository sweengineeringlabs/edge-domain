//! `Query` trait — a read operation that never mutates domain state.

use futures::future::BoxFuture;

use crate::api::query::QueryError;

/// A named read operation that returns data without mutating state.
pub trait Query: Send + Sync {
    /// The type returned on success.
    type Result: Send + 'static;

    /// Stable name identifying this query type.
    fn name(&self) -> &str {
        "query"
    }

    /// Execute the query and return the result.
    fn execute(&self) -> BoxFuture<'_, Result<Self::Result, QueryError>>;
}
