//! `ProviderEndpoint` — connects the provider primitive to the edge dispatch pipeline.

use std::sync::Arc;

use crate::api::provider::traits::ExecutionModel;

/// Pipeline endpoint for the provider primitive.
///
/// Wraps an [`ExecutionModel`] trait object and exposes it as a dispatchable
/// `Handler` (ADR-024).
pub struct ProviderEndpoint {
    pub(crate) model: Arc<dyn ExecutionModel>,
}

impl ProviderEndpoint {
    /// Construct an endpoint over the given execution model.
    pub fn new(model: Arc<dyn ExecutionModel>) -> Self {
        Self { model }
    }
}
