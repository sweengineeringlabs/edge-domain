//! `ProviderFactory` — constructor contract for the default provider primitives.

use crate::api::provider::types::{
    BufferedStreamHandler, EchoExecutionModel, ExecutionConfig, ExecutionConfigBuilder, ModelInfo,
    ModelInfoBuilder, ProviderConfig, ProviderConfigBuilder, ProviderEndpoint, StaticProvider,
    StdProviderFactory, TokenUsageBuilder, ToolCallDeltaBuilder,
};

/// Factory for the standard reference implementations.
///
/// Implement on any unit struct to gain the standard constructors.
pub trait ProviderFactory {
    /// Return the standard provider-factory instance.
    fn std_factory() -> StdProviderFactory {
        StdProviderFactory
    }

    /// Start a fluent [`ExecutionConfigBuilder`].
    fn execution_config_builder() -> ExecutionConfigBuilder {
        ExecutionConfigBuilder::new()
    }

    /// Start a fluent [`ProviderConfigBuilder`].
    fn provider_config_builder() -> ProviderConfigBuilder {
        ProviderConfigBuilder::new()
    }

    /// Start a fluent [`ModelInfoBuilder`].
    fn model_info_builder() -> ModelInfoBuilder {
        ModelInfoBuilder::new()
    }

    /// Start a fluent [`TokenUsageBuilder`].
    fn token_usage_builder() -> TokenUsageBuilder {
        TokenUsageBuilder::new()
    }

    /// Start a fluent [`ToolCallDeltaBuilder`] at the given stream index.
    fn tool_call_delta_builder(index: usize) -> ToolCallDeltaBuilder {
        ToolCallDeltaBuilder::new(index)
    }

    /// Construct the reference [`StaticProvider`] from config and model metadata.
    fn provider(config: ProviderConfig, model: ModelInfo) -> StaticProvider {
        StaticProvider::new(config, model)
    }

    /// Construct the reference [`EchoExecutionModel`] from execution config.
    fn execution_model(config: ExecutionConfig) -> EchoExecutionModel {
        EchoExecutionModel::new(config)
    }

    /// Construct an empty reference [`BufferedStreamHandler`].
    fn stream_handler() -> BufferedStreamHandler {
        BufferedStreamHandler::new()
    }

    /// Construct a pipeline [`ProviderEndpoint`] (the connected Handler + Service
    /// face) over an execution model built from `config`.
    fn endpoint(config: ExecutionConfig) -> ProviderEndpoint {
        ProviderEndpoint::new(EchoExecutionModel::new(config))
    }
}
