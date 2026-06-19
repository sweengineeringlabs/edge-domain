use serde::{Deserialize, Serialize};

use crate::api::complete::types::ToolCallDelta;

/// Incremental update payload inside a [`StreamChunk`](crate::api::complete::types::StreamChunk).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamDelta {
    /// New text content fragment, if any.
    pub content: Option<String>,
    /// Partial tool-call fragments, if any.
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

impl StreamDelta {
    /// Construct a text-only delta.
    pub fn text(content: impl Into<String>) -> Self {
        Self { content: Some(content.into()), tool_calls: None }
    }

    /// Construct an empty delta (heartbeat / keep-alive).
    pub fn empty() -> Self {
        Self::default()
    }
}
