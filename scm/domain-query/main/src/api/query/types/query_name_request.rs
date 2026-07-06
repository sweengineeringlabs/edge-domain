//! [`QueryNameRequest`] — zero-sized marker for querying a query's stable name.

/// Request for a [`Query`](crate::api::query::traits::Query)'s stable name.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct QueryNameRequest;
