//! `Query` trait — a read operation that never mutates domain state.

use futures::future::BoxFuture;

use crate::api::query::QueryError;

/// A named read operation that returns data without mutating state.
///
/// Queries are the read side of the CQRS split.  They must never produce
/// side effects — the same query with the same inputs must be safe to run
/// multiple times.
///
/// ```rust,ignore
/// struct GetOrder { order_id: String }
///
/// impl Query for GetOrder {
///     type Result = Order;
///
///     fn name(&self) -> &str { "get-order" }
///     fn execute(&self) -> BoxFuture<'_, Result<Order, QueryError>> {
///         Box::pin(async move {
///             // read state, never mutate
///         })
///     }
/// }
/// ```
pub trait Query: Send + Sync {
    /// The result type returned by this query.
    type Result: Send + 'static;

    /// Stable name identifying this query type.
    fn name(&self) -> &str {
        "query"
    }

    /// Execute the query and return the result.
    fn execute(&self) -> BoxFuture<'_, Result<Self::Result, QueryError>>;
}
