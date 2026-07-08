use serde::{Deserialize, Serialize};

use crate::api::provider::types::{CompletionMessage, ExecutionConfig, ToolDefinition};

/// Structured input for a single completion or streaming request.
///
/// Carried by [`ProviderCompleteRequest`](super::ProviderCompleteRequest), the request type of
/// [`Provider::complete`](crate::api::provider::traits::Provider::complete), which converts it
/// into an [`edge_llm_complete::CompletionRequest`] and delegates to the provider's
/// [`Completer`](edge_llm_complete::Completer).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionInput {
    /// Ordered conversation history (user / assistant / tool turns).
    pub messages: Vec<CompletionMessage>,
    /// Tools the model may invoke. Empty when tool use is not needed.
    pub tools: Vec<ToolDefinition>,
    /// Optional system prompt prepended before the conversation.
    pub system: Option<String>,
    /// Execution configuration (token cap, timeout, mode).
    pub config: ExecutionConfig,
}
