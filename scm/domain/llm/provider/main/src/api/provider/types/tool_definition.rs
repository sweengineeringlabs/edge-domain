use serde::{Deserialize, Serialize};

use crate::api::provider::types::JsonValue;

/// A tool (function) the model may invoke during completion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Stable identifier the model uses to invoke this tool.
    pub name: String,
    /// Human-readable description guiding when to use the tool.
    pub description: String,
    /// JSON Schema describing the tool's input parameters.
    pub input_schema: JsonValue,
}
