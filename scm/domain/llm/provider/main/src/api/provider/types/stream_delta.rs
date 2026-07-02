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
