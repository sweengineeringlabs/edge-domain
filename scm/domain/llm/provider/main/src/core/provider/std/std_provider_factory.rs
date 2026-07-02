//! `ProviderBootstrap` impl for `StdProviderFactory`.

use std::sync::Arc;

use edge_domain_observer::ObserverContext;
use edge_llm_complete::Completer;

use crate::api::{
    ModelInfo, Provider, ProviderBootstrap, ProviderConfig, StdProvider, StdProviderFactory,
};

impl ProviderBootstrap for StdProviderFactory {
    fn provider(
        config: ProviderConfig,
        model: Box<ModelInfo>,
        completer: Arc<dyn Completer>,
        observer: Arc<dyn ObserverContext>,
    ) -> Arc<dyn Provider> {
        Arc::new(StdProvider::new(config, *model, completer, observer))
    }
}
