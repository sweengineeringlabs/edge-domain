use crate::api::provider::types::ToolCallDelta;
use serde::{Deserialize, Serialize};

/// Incremental update in a streamed response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamDelta {
    /// Text content (if text streaming)
    pub content: Option<String>,

    /// Tool call deltas (if tool streaming)
    pub tool_calls: Vec<ToolCallDelta>,
}

impl StreamDelta {
    /// Create a text delta
    pub fn text(content: String) -> Self {
        Self {
            content: Some(content),
            tool_calls: vec![],
        }
    }

    /// Create a tool call delta
    pub fn tool_calls(calls: Vec<ToolCallDelta>) -> Self {
        Self {
            content: None,
            tool_calls: calls,
        }
    }

    /// Create an empty delta
    pub fn empty() -> Self {
        Self {
            content: None,
            tool_calls: vec![],
        }
    }

    /// Check if delta has content
    pub fn is_empty(&self) -> bool {
        self.content.is_none() && self.tool_calls.is_empty()
    }
}
