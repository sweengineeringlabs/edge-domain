use serde::{Deserialize, Serialize};

use crate::api::provider::types::{CompletionMessage, ExecutionConfig, ToolDefinition};

/// Structured input for a single completion or streaming request.
///
/// Orphan-type note: `Provider`'s current trait methods don't take this type directly (no
/// `complete`/`stream` methods exist on the trait) — it's constructed by callers and passed to
/// `edge_llm_complete::Completer` implementations instead. Plain data struct, no interface
/// behind it — inventing a trait solely to reference it would be ceremony with no real
/// polymorphism.
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
