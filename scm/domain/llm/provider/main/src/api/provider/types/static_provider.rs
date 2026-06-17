//! `StaticProvider` — reference [`Provider`](crate::api::provider::traits::Provider) implementation.

use crate::api::provider::types::{ModelInfo, ProviderConfig};

/// Reference provider that reports static configuration and metadata.
///
/// This is a domain primitive with no network backend: it surfaces the
/// configuration and model metadata it was constructed with, so callers can
/// exercise the [`Provider`](crate::api::provider::traits::Provider) contract
/// deterministically in tests and wiring.
#[derive(Clone, Debug)]
pub struct StaticProvider {
    pub(crate) config: ProviderConfig,
    pub(crate) model: ModelInfo,
}

impl StaticProvider {
    /// Construct a provider that reports the given config and model metadata.
    pub fn new(config: ProviderConfig, model: ModelInfo) -> Self {
        Self { config, model }
    }
}
