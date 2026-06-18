use serde::{Deserialize, Serialize};

/// Reason a completion finished
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum FinishReason {
    /// Model reached natural stop
    #[serde(rename = "stop")]
    Stop,

    /// Max tokens reached
    #[serde(rename = "length")]
    Length,

    /// Tool call requested
    #[serde(rename = "tool_calls")]
    ToolCalls,

    /// Content filter triggered
    #[serde(rename = "content_filter")]
    ContentFilter,

    /// Unexpected end (error)
    #[serde(rename = "error")]
    Error,
}
