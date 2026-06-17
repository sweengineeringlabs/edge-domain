//! `StdPromptFactory` — reference [`PromptFactory`](crate::api::prompt::traits::PromptFactory) implementation.

/// Reference implementation of [`PromptFactory`](crate::api::prompt::traits::PromptFactory).
///
/// Implement the factory trait on this unit struct to gain the standard
/// constructors for the default prompt primitives.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdPromptFactory;
