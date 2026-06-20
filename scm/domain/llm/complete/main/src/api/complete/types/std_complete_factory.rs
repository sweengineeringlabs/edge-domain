//! `StdCompleteFactory` — standard [`CompleteBootstrap`](crate::api::complete::traits::CompleteBootstrap) implementation.

/// Unit struct that implements [`CompleteBootstrap`](crate::api::complete::traits::CompleteBootstrap)
/// via all-default method bodies.
///
/// Consumers call `StdCompleteFactory::user_message(...)` etc. to get idiomatic
/// completion primitives without naming any concrete type from `core/`.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdCompleteFactory;
