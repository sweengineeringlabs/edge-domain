use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool (function) the model may invoke during completion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Stable identifier the model uses to invoke this tool.
    pub name: String,
    /// Human-readable description guiding when to use the tool.
    pub description: String,
    /// JSON Schema describing the tool's input parameters.
    pub input_schema: Value,
}

impl ToolDefinition {
    /// Construct a tool definition.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
        }
    }
}
