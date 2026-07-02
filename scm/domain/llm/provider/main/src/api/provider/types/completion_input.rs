use serde::{Deserialize, Serialize};

use crate::api::provider::types::{CompletionMessage, ExecutionConfig, ToolDefinition};

/// Structured input for a single completion or streaming request.
///
/// Passed to [`crate::Provider::complete`] and [`crate::Provider::stream`].
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
