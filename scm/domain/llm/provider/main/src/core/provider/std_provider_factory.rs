//! `ProviderBootstrap` impl for `StdProviderFactory`.

use std::sync::Arc;

use edge_domain_observer::ObserverContext;
use edge_llm_complete::Completer;

use crate::api::{
    ModelInfo, Provider, ProviderBootstrap, ProviderConfig, ProviderCore, StdProviderFactory,
};

impl ProviderBootstrap for StdProviderFactory {
    fn provider(
        config: ProviderConfig,
        model: ModelInfo,
        completer: Arc<dyn Completer>,
        observer: Arc<dyn ObserverContext>,
    ) -> Arc<dyn Provider> {
        Arc::new(ProviderCore::new(config, model, completer, observer))
    }
}
