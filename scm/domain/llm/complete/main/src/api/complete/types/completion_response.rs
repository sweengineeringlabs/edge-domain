use serde::{Deserialize, Serialize};

use crate::api::complete::types::{FinishReason, TokenUsage, ToolCall};

/// The result of a non-streaming completion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompletionResponse {
    /// Provider-assigned response identifier.
    pub id: String,
    /// Model that produced this response.
    pub model: String,
    /// Generated text content, if any.
    pub content: Option<String>,
    /// Tool calls requested by the model, if any.
    pub tool_calls: Vec<ToolCall>,
    /// Why generation stopped.
    pub finish_reason: FinishReason,
    /// Token consumption for this request.
    pub usage: Box<TokenUsage>,
}
