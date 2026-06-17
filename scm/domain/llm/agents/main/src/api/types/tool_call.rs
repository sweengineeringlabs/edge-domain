use serde::{Deserialize, Serialize};

/// Tool call within a message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier of the tool call.
    pub id: String,
    /// Name of the tool to invoke.
    pub name: String,
    /// Serialized JSON arguments for the tool call.
    pub arguments: String,
}
