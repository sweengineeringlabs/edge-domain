use serde::{Deserialize, Serialize};

/// A tool invocation emitted by the model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ToolCall {
    /// Unique identifier for this invocation.
    pub id: String,
    /// Name of the tool to call.
    pub name: String,
    /// JSON-encoded arguments string.
    pub arguments: String,
}
