//! `ProviderCore` — concrete implementation type for the standard provider.

use std::sync::Arc;

use edge_domain_observer::ObserverContext;
use edge_llm_complete::Completer;

use crate::api::provider::types::{ModelInfo, ProviderConfig};

/// Concrete [`Provider`](crate::api::provider::traits::Provider) produced by
/// [`ProviderBootstrap::provider`](crate::api::provider::traits::ProviderBootstrap::provider).
///
/// Not part of the public crate API — callers depend on the `Provider` trait,
/// not this type. Declared in `api/` to satisfy SEA module correspondence rules.
pub struct ProviderCore {
    pub(crate) config: ProviderConfig,
    pub(crate) model: Option<ModelInfo>,
    pub(crate) completer: Arc<dyn Completer>,
    pub(crate) observer: Arc<dyn ObserverContext>,
}

impl ProviderCore {
    /// Construct a provider core from config, model metadata, a completer delegate, and an observer.
    pub(crate) fn new(
        config: ProviderConfig,
        model: ModelInfo,
        completer: Arc<dyn Completer>,
        observer: Arc<dyn ObserverContext>,
    ) -> Self {
        Self {
            config,
            model: Some(model),
            completer,
            observer,
        }
    }
}
