//! `StdCompleteFactory` — standard [`CompleteFactory`](crate::api::complete::traits::CompleteFactory) implementation.

/// Unit struct that implements [`CompleteFactory`](crate::api::complete::traits::CompleteFactory)
/// via all-default method bodies.
///
/// Consumers call `StdCompleteFactory::user_message(...)` etc. to get idiomatic
/// completion primitives without naming any concrete type from `core/`.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdCompleteFactory;
