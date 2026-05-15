//! `Query` trait — a read operation that never mutates domain state.

use async_trait::async_trait;

use crate::api::query_error::QueryError;

/// A named read operation that returns data without mutating state.
///
/// Queries are the read side of the CQRS split.  They must never produce
/// side effects — the same query with the same inputs must be safe to run
/// multiple times.
///
/// ```rust,ignore
/// struct GetOrder { order_id: String }
///
/// #[async_trait]
/// impl Query<Order> for GetOrder {
///     fn name(&self) -> &str { "get-order" }
///     async fn execute(&self) -> Result<Order, QueryError> {
///         // read state, never mutate
///     }
/// }
/// ```
#[async_trait]
pub trait Query<R: Send + 'static>: Send + Sync {
    /// Stable name identifying this query type.
    fn name(&self) -> &str;

    /// Execute the query and return the result.
    async fn execute(&self) -> Result<R, QueryError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_is_object_safe() {
        fn _assert(_: &dyn Query<String>) {}
    }
}
