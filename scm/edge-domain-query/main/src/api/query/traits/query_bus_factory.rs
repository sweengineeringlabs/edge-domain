//! [`QueryBusFactory`] — constructor contract for query bus implementations.

use crate::api::query::types::DirectQueryBus;
use crate::api::query::types::NoopQuery;
use crate::api::query::types::StdQueryBusFactory;

/// Factory trait for the standard [`QueryBus`](crate::api::query::traits::QueryBus) implementations.
pub trait QueryBusFactory {
    /// Construct the inline [`DirectQueryBus`] that dispatches queries without queuing.
    fn direct<R: Send + 'static>() -> DirectQueryBus<R> {
        DirectQueryBus::new()
    }

    /// Construct the standard [`StdQueryBusFactory`] implementation.
    fn std() -> StdQueryBusFactory {
        StdQueryBusFactory
    }

    /// Construct a [`NoopQuery`] that always succeeds with a `()` result.
    fn noop_query() -> NoopQuery {
        NoopQuery
    }
}
