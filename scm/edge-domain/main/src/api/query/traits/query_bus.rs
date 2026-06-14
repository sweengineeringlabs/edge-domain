//! `QueryBus` trait — dispatches queries and returns their results.

use futures::future::BoxFuture;

use super::query::Query;
use crate::api::query::QueryError;

/// Dispatches [`Query`] instances and returns their results.
///
/// The bus decouples the caller from the query implementation.
///
/// ```rust,ignore
/// impl QueryBus for DirectQueryBus<Order> {
///     type Result = Order;
///
///     fn dispatch(&self, query: Box<dyn Query<Result = Order>>) -> BoxFuture<'_, Result<Order, QueryError>> {
///         Box::pin(async move { query.execute().await })
///     }
/// }
/// ```
pub trait QueryBus: Send + Sync {
    /// The result type returned by queries dispatched through this bus.
    type Result: Send + 'static;

    /// Dispatch a query and return its result.
    fn dispatch(&self, query: Box<dyn Query<Result = Self::Result>>) -> BoxFuture<'_, Result<Self::Result, QueryError>>;
}
