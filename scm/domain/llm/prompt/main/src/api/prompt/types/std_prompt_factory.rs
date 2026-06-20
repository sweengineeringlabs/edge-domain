//! `StdPromptFactory` — reference [`PromptBootstrap`](crate::api::prompt::traits::PromptBootstrap) implementation.

/// Reference implementation of [`PromptBootstrap`](crate::api::prompt::traits::PromptBootstrap).
///
/// Implement the bootstrap trait on this unit struct to gain the standard
/// constructors for the default prompt primitives.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdPromptFactory;
