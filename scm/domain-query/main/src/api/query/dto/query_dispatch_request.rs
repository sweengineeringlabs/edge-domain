//! [`QueryDispatchRequest`] — request to dispatch a query through a bus.

use crate::api::query::traits::Query;

/// Request to dispatch `query` through a [`QueryBus`](crate::api::query::traits::QueryBus).
pub struct QueryDispatchRequest<R> {
    /// The query to dispatch.
    pub query: Box<dyn Query<Result = R>>,
}
