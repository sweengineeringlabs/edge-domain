//! `StdCommandBusFactory` — the canonical [`CommandBusBootstrap`](crate::CommandBusBootstrap) marker.

/// Canonical marker that implements the standard [`CommandBusBootstrap`](crate::CommandBusBootstrap) contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdCommandBusFactory;
