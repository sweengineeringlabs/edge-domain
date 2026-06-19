use serde::{Deserialize, Serialize};

/// Reason a completion generation ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Model reached a natural stop point.
    #[default]
    Stop,
    /// Token limit was reached.
    Length,
    /// Model requested one or more tool calls.
    ToolCalls,
    /// Content was filtered by a safety system.
    ContentFilter,
    /// Generation ended with an error.
    Error,
}
