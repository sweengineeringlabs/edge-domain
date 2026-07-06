//! [`ProjectionReadModelRequest`] — zero-sized marker for querying the read model.

/// Request for a [`Projection`](crate::api::projection::traits::Projection)'s current read model.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionReadModelRequest;
