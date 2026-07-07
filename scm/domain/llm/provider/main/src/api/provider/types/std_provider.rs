//! `StdProvider` — concrete implementation type for the standard provider.

use std::sync::Arc;

use edge_domain_observer::ObserverContext;
use edge_llm_complete::Completer;

use crate::api::provider::types::{ModelInfo, ProviderConfig};

/// Concrete [`Provider`](crate::api::provider::traits::Provider) reference implementation.
/// Construct via `StdProvider::new(config, model, completer, observer)`.
///
/// Not part of the intended public *contract* — callers depend on the `Provider` trait,
/// not this type — but `new` is `pub` since it's the only constructor.
pub struct StdProvider {
    pub(crate) config: ProviderConfig,
    pub(crate) model: Option<ModelInfo>,
    pub(crate) completer: Arc<dyn Completer>,
    pub(crate) observer: Arc<dyn ObserverContext>,
}
