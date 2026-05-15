//! `QueryBus` trait — dispatches queries and returns their results.

use async_trait::async_trait;

use crate::api::query::Query;
use crate::api::query::QueryError;

/// Dispatches [`Query`] instances and returns their results.
///
/// The bus decouples the caller from the query implementation.
///
/// ```rust,ignore
/// #[async_trait]
/// impl QueryBus<Order> for DirectQueryBus<Order> {
///     async fn dispatch(&self, query: Box<dyn Query<Order>>) -> Result<Order, QueryError> {
///         query.execute().await
///     }
/// }
/// ```
#[async_trait]
pub trait QueryBus<R: Send + 'static>: Send + Sync {
    /// Dispatch a query and return its result.
    async fn dispatch(&self, query: Box<dyn Query<R>>) -> Result<R, QueryError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_bus_is_object_safe() {
        fn _assert(_: &dyn QueryBus<String>) {}
    }
}
