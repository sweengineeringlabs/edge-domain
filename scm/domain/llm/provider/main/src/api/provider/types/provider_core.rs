//! `ProviderCore` — concrete implementation type for the standard provider.

use std::sync::Arc;

use edge_llm_complete::Completer;

use crate::api::provider::types::{ModelInfo, ProviderConfig};

/// Concrete [`Provider`](crate::api::provider::traits::Provider) produced by
/// [`ProviderFactory::provider`](crate::api::provider::traits::ProviderFactory::provider).
///
/// Not part of the public crate API — callers depend on the `Provider` trait,
/// not this type. Declared in `api/` to satisfy SEA module correspondence rules.
pub struct ProviderCore {
    pub(crate) config: ProviderConfig,
    pub(crate) model: Option<ModelInfo>,
    pub(crate) completer: Arc<dyn Completer>,
}

impl ProviderCore {
    /// Construct a provider core from config, model metadata, and a completer delegate.
    pub(crate) fn new(config: ProviderConfig, model: ModelInfo, completer: Arc<dyn Completer>) -> Self {
        Self { config, model: Some(model), completer }
    }
}
