//! `ProviderFactory` impl for `StdProviderFactory`.

use std::sync::Arc;

use edge_llm_complete::Completer;

use crate::api::{ModelInfo, ProviderConfig, ProviderCore, ProviderFactory, StdProviderFactory};

impl ProviderFactory for StdProviderFactory {
    fn provider(config: ProviderConfig, model: ModelInfo, completer: Arc<dyn Completer>) -> ProviderCore {
        ProviderCore::new(config, model, completer)
    }
}
