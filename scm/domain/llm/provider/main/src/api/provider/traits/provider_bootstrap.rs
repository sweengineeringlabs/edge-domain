//! `ProviderBootstrap` — constructor contract for the default provider primitives.

use std::sync::Arc;

use edge_domain_observer::ObserveContext;
use edge_llm_complete::Completer;
use serde_json::Value;

use crate::api::provider::traits::Provider;
use crate::api::provider::types::{
    BufferedStreamHandler, CompletionInput, CompletionMessage, EchoExecutionModel,
    EchoProviderCompleter, ExecutionConfig, MessageRole, ModelInfo, ProviderConfig,
    StdProviderFactory, ToolDefinition,
};

/// Constructor namespace for the standard provider reference implementations.
///
/// Implement on any unit struct to gain the standard constructors.
pub trait ProviderBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "provider"
    }

    /// Return the standard provider-factory instance.
    fn std_factory() -> StdProviderFactory
    where
        Self: Sized,
    {
        StdProviderFactory
    }

    /// Construct a [`Provider`] from config, model metadata, a completer delegate, and an observer.
    fn provider(
        config: ProviderConfig,
        model: ModelInfo,
        completer: Arc<dyn Completer>,
        observer: Arc<dyn ObserveContext>,
    ) -> Arc<dyn Provider>
    where
        Self: Sized;

    /// Construct the reference [`EchoExecutionModel`] from execution config.
    fn execution_model(config: ExecutionConfig) -> EchoExecutionModel
    where
        Self: Sized,
    {
        EchoExecutionModel::new(config)
    }

    /// Construct an empty reference [`BufferedStreamHandler`].
    fn stream_handler() -> BufferedStreamHandler
    where
        Self: Sized,
    {
        BufferedStreamHandler::new()
    }

    /// Construct a [`CompletionMessage`] with the given role and content.
    fn message(role: MessageRole, content: impl Into<String>) -> CompletionMessage
    where
        Self: Sized,
    {
        CompletionMessage {
            role,
            content: content.into(),
        }
    }

    /// Construct a [`ToolDefinition`] with the given name, description, and schema.
    fn tool(
        name: impl Into<String>,
        description: impl Into<String>,
        schema: Value,
    ) -> ToolDefinition
    where
        Self: Sized,
    {
        ToolDefinition::new(name, description, schema)
    }

    /// Construct a [`CompletionInput`] from messages, tools, system prompt, and config.
    fn completion_input(
        messages: Vec<CompletionMessage>,
        tools: Vec<ToolDefinition>,
        system: Option<String>,
        config: ExecutionConfig,
    ) -> CompletionInput
    where
        Self: Sized,
    {
        CompletionInput::new(messages, tools, system, config)
    }

    /// Construct the default [`EchoProviderCompleter`] adapter.
    ///
    /// This adapter implements [`edge_llm_complete::Completer`] by delegating to the
    /// provider's `EchoExecutionModel`, mapping request/response across the port boundary.
    fn provider_completer() -> EchoProviderCompleter
    where
        Self: Sized,
    {
        EchoProviderCompleter
    }
}
