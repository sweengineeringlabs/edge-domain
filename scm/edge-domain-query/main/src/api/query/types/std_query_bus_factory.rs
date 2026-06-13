//! `StdQueryBusFactory` — standard concrete [`QueryBusFactory`](crate::api::query::traits::QueryBusFactory).

/// Marker type for the standard [`QueryBusFactory`](crate::api::query::traits::QueryBusFactory)
/// implementation that constructs the built-in query bus variants.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdQueryBusFactory;
