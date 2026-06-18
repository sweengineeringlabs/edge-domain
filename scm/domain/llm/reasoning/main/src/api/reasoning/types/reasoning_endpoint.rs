//! `ReasoningEndpoint` — connects the reasoning primitive to the edge dispatch pipeline.

use std::sync::Arc;

use crate::api::reasoning::traits::Reasoning;

/// Pipeline endpoint for the reasoning primitive.
///
/// Wraps a [`Reasoning`] trait object and exposes it as a dispatchable
/// `Handler` (ADR-024).
pub struct ReasoningEndpoint {
    pub(crate) reasoner: Arc<dyn Reasoning>,
}

impl ReasoningEndpoint {
    /// Construct an endpoint over the given reference reasoner.
    pub fn new(reasoner: Arc<dyn Reasoning>) -> Self {
        Self { reasoner }
    }
}
