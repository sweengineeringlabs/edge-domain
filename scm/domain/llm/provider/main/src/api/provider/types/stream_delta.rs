use crate::api::provider::types::ToolCallDelta;
use serde::{Deserialize, Serialize};

/// Incremental update in a streamed response.
///
/// A delta is one of three exclusive variants:
/// - [`StreamDelta::Text`] — a text fragment
/// - [`StreamDelta::ToolCalls`] — one or more tool call deltas
/// - [`StreamDelta::Empty`] — no content (e.g. a heartbeat chunk)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum StreamDelta {
    /// Text content fragment
    Text(String),
    /// Tool call deltas
    ToolCalls(Vec<ToolCallDelta>),
    /// Empty delta
    Empty,
}

impl StreamDelta {
    /// Create a text delta
    pub fn text(content: String) -> Self {
        Self::Text(content)
    }

    /// Create a tool call delta
    pub fn tool_calls(calls: Vec<ToolCallDelta>) -> Self {
        Self::ToolCalls(calls)
    }

    /// Create an empty delta
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Check if delta carries no content
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
