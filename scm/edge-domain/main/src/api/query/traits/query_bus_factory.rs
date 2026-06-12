//! [`QueryBusFactory`] — constructor contract for query bus implementations.

use crate::api::query::types::direct_query_bus::DirectQueryBus;

/// Factory trait for the standard [`QueryBus`](crate::api::query::traits::query_bus::QueryBus) implementations.
pub trait QueryBusFactory {
    /// Construct the inline [`DirectQueryBus`] that dispatches queries without queuing.
    fn direct() -> DirectQueryBus {
        DirectQueryBus
    }
}
