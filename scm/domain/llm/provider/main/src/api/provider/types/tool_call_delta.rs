use serde::{Deserialize, Serialize};

/// Incremental tool call in a streamed response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Tool call index
    pub index: usize,

    /// Tool call ID
    pub id: Option<String>,

    /// Tool name
    pub name: Option<String>,

    /// Partial arguments (JSON string fragment)
    pub arguments: Option<String>,
}
