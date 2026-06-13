//! `QueryBus` trait — dispatches queries and returns their results.

use futures::future::BoxFuture;

use crate::api::query::traits::Query;
use crate::api::query::QueryError;

/// Dispatches [`Query`] instances and returns their results.
pub trait QueryBus: Send + Sync {
    /// The result type produced by queries dispatched through this bus.
    type Result: Send + 'static;

    /// Dispatch a query and return its result.
    fn dispatch(
        &self,
        query: Box<dyn Query<Result = Self::Result>>,
    ) -> BoxFuture<'_, Result<Self::Result, QueryError>>;
}
