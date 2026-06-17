//! `StdReasoningFactory` — reference [`ReasoningFactory`](crate::api::reasoning::traits::ReasoningFactory) implementation.

/// Reference implementation of [`ReasoningFactory`](crate::api::reasoning::traits::ReasoningFactory).
///
/// Implement the factory trait on this unit struct to gain the standard
/// constructors for the default reasoning primitives.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdReasoningFactory;
