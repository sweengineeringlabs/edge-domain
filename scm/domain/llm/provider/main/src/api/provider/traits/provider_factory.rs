//! `ProviderFactory` — constructor contract for the default provider primitives.

use crate::api::provider::types::{
    BufferedStreamHandler, EchoExecutionModel, ExecutionConfig, ModelInfo, ProviderConfig,
    StaticProvider, StdProviderFactory,
};

/// Factory for the standard reference implementations.
///
/// Implement on any unit struct to gain the standard constructors.
pub trait ProviderFactory {
    /// Return the standard provider-factory instance.
    fn std_factory() -> StdProviderFactory {
        StdProviderFactory
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
}
