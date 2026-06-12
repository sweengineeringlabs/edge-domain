//! `QueryBus` trait — dispatches queries and returns their results.

use futures::future::BoxFuture;

use crate::api::query::traits::Query;
use crate::api::query::QueryError;

/// Dispatches [`Query`] instances and returns their results.
pub trait QueryBus<R: Send + 'static>: Send + Sync {
    /// Dispatch a query and return its result.
    fn dispatch(&self, query: Box<dyn Query<R>>) -> BoxFuture<'_, Result<R, QueryError>>;
}
