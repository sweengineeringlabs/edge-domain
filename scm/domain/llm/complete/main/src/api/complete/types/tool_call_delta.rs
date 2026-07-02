use serde::{Deserialize, Serialize};

/// Incremental fragment of a tool call arriving in a stream chunk.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ToolCallDelta {
    /// Position index of the tool call in the model's output list.
    pub index: u32,
    /// Tool call id (present in the first fragment only).
    pub id: Option<String>,
    /// Tool name (present in the first fragment only).
    pub name: Option<String>,
    /// Partial JSON-encoded arguments accumulated so far.
    pub arguments: Option<String>,
}
