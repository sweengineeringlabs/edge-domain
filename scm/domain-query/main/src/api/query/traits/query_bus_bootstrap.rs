//! [`QueryBusBootstrap`] — constructor contract for query bus implementations.

use std::sync::Arc;

use crate::api::query::traits::QueryBus;
use crate::api::query::types::DirectQueryBus;
use crate::api::query::types::LoggingQueryBus;
use crate::api::query::types::NoopQuery;
use crate::api::query::types::NoopQueryBus;
use crate::api::query::types::StdQueryBusFactory;

/// Bootstrap trait for the standard [`QueryBus`](crate::api::query::traits::QueryBus) implementations.
pub trait QueryBusBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "query"
    }

    /// Construct the inline [`DirectQueryBus`] that dispatches queries without queuing.
    fn direct<R: Send + 'static>() -> DirectQueryBus<R> where Self: Sized {
        DirectQueryBus::new()
    }

    /// Construct the standard [`StdQueryBusFactory`] implementation.
    fn std() -> StdQueryBusFactory where Self: Sized {
        StdQueryBusFactory
    }

    /// Construct a [`NoopQuery`] that always succeeds with a `()` result.
    fn noop_query() -> NoopQuery where Self: Sized {
        NoopQuery
    }

    /// Construct a [`NoopQueryBus`] that always returns `Err(QueryError::NotFound)`.
    fn noop_query_bus<R: Send + 'static>() -> NoopQueryBus<R> where Self: Sized {
        NoopQueryBus::new()
    }

    /// Wrap `inner` with a [`LoggingQueryBus`] that records dispatch outcomes via `tracing`.
    fn logging_query<R: Send + 'static>(
        inner: Arc<dyn QueryBus<Result = R>>,
    ) -> LoggingQueryBus<R> where Self: Sized {
        LoggingQueryBus { inner }
    }
}
