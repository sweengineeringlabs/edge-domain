use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{EchoExecutionModel, ExecutionConfig, ExecutionModel, ExecutionStepResult};
use crate::core::provider::DefaultProviderHandler;

impl crate::api::StdProviderFactory {
    /// Construct a dispatchable provider handler backed by the given execution model.
    pub fn provider_handler(
        model: Arc<dyn ExecutionModel>,
    ) -> impl Handler<Request = String, Response = ExecutionStepResult> {
        DefaultProviderHandler { model }
    }

    /// Construct a dispatchable provider handler backed by the reference [`EchoExecutionModel`].
    pub fn default_provider_handler(
        config: ExecutionConfig,
    ) -> impl Handler<Request = String, Response = ExecutionStepResult> {
        Self::provider_handler(Arc::new(EchoExecutionModel::new(config)))
    }
}
