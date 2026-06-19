//! `ProviderFactory` — constructor contract for the default provider primitives.

use std::sync::Arc;

use edge_llm_complete::Completer;
use serde_json::Value;

use crate::api::provider::types::{
    BufferedStreamHandler, CompletionInput, CompletionMessage, EchoProviderCompleter,
    EchoExecutionModel, ExecutionConfig, MessageRole, ModelInfo, ProviderConfig, ProviderCore,
    StdProviderFactory, ToolDefinition,
};

/// Factory for the standard reference implementations.
///
/// Implement on any unit struct to gain the standard constructors.
pub trait ProviderFactory {
    /// Return the standard provider-factory instance.
    fn std_factory() -> StdProviderFactory {
        StdProviderFactory
    }

    /// Construct a [`ProviderCore`] from config, model metadata, and a completer delegate.
    ///
    /// `ProviderCore` is not part of the public crate API — callers use it via the
    /// [`Provider`](crate::api::provider::traits::Provider) trait.
    fn provider(config: ProviderConfig, model: ModelInfo, completer: Arc<dyn Completer>) -> ProviderCore;

    /// Construct the reference [`EchoExecutionModel`] from execution config.
    fn execution_model(config: ExecutionConfig) -> EchoExecutionModel {
        EchoExecutionModel::new(config)
    }

    /// Construct an empty reference [`BufferedStreamHandler`].
    fn stream_handler() -> BufferedStreamHandler {
        BufferedStreamHandler::new()
    }

    /// Construct a [`CompletionMessage`] with the given role and content.
    fn message(role: MessageRole, content: impl Into<String>) -> CompletionMessage {
        CompletionMessage { role, content: content.into() }
    }

    /// Construct a [`ToolDefinition`] with the given name, description, and schema.
    fn tool(name: impl Into<String>, description: impl Into<String>, schema: Value) -> ToolDefinition {
        ToolDefinition::new(name, description, schema)
    }

    /// Construct a [`CompletionInput`] from messages, tools, system prompt, and config.
    fn completion_input(
        messages: Vec<CompletionMessage>,
        tools: Vec<ToolDefinition>,
        system: Option<String>,
        config: ExecutionConfig,
    ) -> CompletionInput {
        CompletionInput::new(messages, tools, system, config)
    }

    /// Construct the default [`EchoProviderCompleter`] adapter.
    ///
    /// This adapter implements [`edge_llm_complete::Completer`] by delegating to the
    /// provider's `EchoExecutionModel`, mapping request/response across the port boundary.
    fn provider_completer() -> EchoProviderCompleter {
        EchoProviderCompleter
    }
}
