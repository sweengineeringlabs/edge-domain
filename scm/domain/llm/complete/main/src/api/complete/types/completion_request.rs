use serde::{Deserialize, Serialize};

use crate::api::complete::types::{Message, ToolChoice, ToolDefinition};

/// A completion request submitted to a [`Completer`](crate::api::complete::traits::Completer).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompletionRequest {
    /// Model identifier (e.g. `"claude-sonnet-4-6"`).
    pub model: String,
    /// Ordered conversation history.
    pub messages: Vec<Message>,
    /// Sampling temperature in `[0.0, 2.0]`.
    pub temperature: Option<f32>,
    /// Hard cap on tokens the model may generate.
    pub max_tokens: Option<u32>,
    /// Nucleus sampling cut-off.
    pub top_p: Option<f32>,
    /// Stop sequences that terminate generation.
    pub stop: Option<Vec<String>>,
    /// Tools the model may invoke.
    pub tools: Option<Vec<ToolDefinition>>,
    /// Tool calling mode.
    pub tool_choice: Option<ToolChoice>,
}
