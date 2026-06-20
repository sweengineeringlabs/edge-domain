//! `StdQueryBusFactory` — standard concrete [`QueryBusBootstrap`](crate::api::query::traits::QueryBusBootstrap).

/// Marker type for the standard [`QueryBusBootstrap`](crate::api::query::traits::QueryBusBootstrap)
/// implementation that constructs the built-in query bus variants.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdQueryBusFactory;
