//! `StdReasoningFactory` — reference [`ReasoningBootstrap`](crate::api::reasoning::traits::ReasoningBootstrap) implementation.

/// Reference implementation of [`ReasoningBootstrap`](crate::api::reasoning::traits::ReasoningBootstrap).
///
/// Implement the bootstrap trait on this unit struct to gain the standard
/// constructors for the default reasoning primitives.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdReasoningFactory;
