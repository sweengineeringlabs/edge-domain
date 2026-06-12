//! [`QueryBusFactory`] — constructor contract for query bus implementations.

use crate::api::query::types::DirectQueryBus;

/// Factory trait for the standard [`QueryBus`](crate::api::query::traits::QueryBus) implementations.
pub trait QueryBusFactory {
    /// Construct the inline [`DirectQueryBus`] that dispatches queries without queuing.
    fn direct() -> DirectQueryBus {
        DirectQueryBus
    }
}
