//! `QueryBus` trait — dispatches queries and returns their results.

use futures::future::BoxFuture;

use super::query::Query;
use crate::api::query::QueryError;

/// Dispatches [`Query`] instances and returns their results.
///
/// The bus decouples the caller from the query implementation.
///
/// ```rust,ignore
/// impl QueryBus<Order> for DirectQueryBus<Order> {
///     fn dispatch(&self, query: Box<dyn Query<Order>>) -> BoxFuture<'_, Result<Order, QueryError>> {
///         Box::pin(async move { query.execute().await })
///     }
/// }
/// ```
pub trait QueryBus<R: Send + 'static>: Send + Sync {
    /// Dispatch a query and return its result.
    fn dispatch(&self, query: Box<dyn Query<R>>) -> BoxFuture<'_, Result<R, QueryError>>;
}
