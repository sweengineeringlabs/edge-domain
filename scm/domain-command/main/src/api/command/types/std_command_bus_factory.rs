//! `StdCommandBusFactory` — the canonical [`CommandBusFactory`](crate::CommandBusFactory) marker.

/// Canonical marker that implements the standard [`CommandBusFactory`](crate::CommandBusFactory) contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdCommandBusFactory;
