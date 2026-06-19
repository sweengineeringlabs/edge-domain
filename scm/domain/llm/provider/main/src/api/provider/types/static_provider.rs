//! `StaticProvider` — reference [`Provider`](crate::api::provider::traits::Provider) implementation.

use std::fmt;
use std::sync::Arc;

use edge_llm_complete::Completer;

use crate::api::provider::types::{ModelInfo, ProviderConfig};

/// Reference provider that reports static configuration and metadata.
///
/// Holds an inner [`Completer`] that it delegates HTTP completion calls to.
/// The reference impl uses [`edge_llm_complete::NoopCompleter`] so tests and
/// wiring exercises can run without a live backend.
#[derive(Clone)]
pub struct StaticProvider {
    pub(crate) config: ProviderConfig,
    pub(crate) model: Option<ModelInfo>,
    pub(crate) completer: Arc<dyn Completer>,
}

impl StaticProvider {
    /// Construct a provider with the given config, model metadata, and completer delegate.
    pub fn new(config: ProviderConfig, model: ModelInfo, completer: Arc<dyn Completer>) -> Self {
        Self { config, model: Some(model), completer }
    }
}

impl fmt::Debug for StaticProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StaticProvider")
            .field("config", &self.config)
            .field("model", &self.model)
            .field("completer", &"Arc<dyn Completer>")
            .finish()
    }
}
