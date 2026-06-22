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

impl ToolCallDelta {
    /// Construct a tool-call delta at the given stream position.
    pub fn new(index: u32) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }

    /// Attach the tool id to this delta.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Attach the tool name to this delta.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Attach a partial arguments string to this delta.
    pub fn with_arguments(mut self, arguments: impl Into<String>) -> Self {
        self.arguments = Some(arguments.into());
        self
    }
}
