//! `PromptEndpoint` — connects the prompt primitive to the edge dispatch pipeline.

use std::sync::Arc;

use crate::api::prompt::traits::Prompt;

/// Pipeline endpoint for the prompt primitive.
///
/// Wraps a [`Prompt`] trait object and exposes it as a dispatchable
/// `Handler` (ADR-024).
pub struct PromptEndpoint {
    pub(crate) prompt: Arc<dyn Prompt>,
}

impl PromptEndpoint {
    /// Construct an endpoint over the given reference prompt.
    pub fn new(prompt: Arc<dyn Prompt>) -> Self {
        Self { prompt }
    }
}
