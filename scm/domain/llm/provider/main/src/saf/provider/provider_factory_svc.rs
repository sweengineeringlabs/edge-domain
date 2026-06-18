use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{EchoExecutionModel, ExecutionConfig, ExecutionModel, ExecutionStepResult};
use crate::core::provider::default_provider::DefaultProviderHandler;

pub use crate::api::ExecutionConfigBuilder;
pub use crate::api::ModelInfoBuilder;
pub use crate::api::ProviderConfigBuilder;
pub use crate::api::ProviderFactory;
pub use crate::api::StdProviderFactory;
pub use crate::api::TokenUsageBuilder;
pub use crate::api::ToolCallDeltaBuilder;

/// SAF contract identifier for the provider-factory service.
pub const PROVIDER_FACTORY_SVC: &str = "provider_factory";

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
    provider_handler(Arc::new(EchoExecutionModel::new(config)))
}
