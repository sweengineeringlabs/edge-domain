use serde::{Deserialize, Serialize};

/// Type of a prompt variable
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum VariableType {
    /// Plain text string
    #[serde(rename = "string")]
    String,

    /// Numeric value (integer or float)
    #[serde(rename = "number")]
    Number,

    /// Boolean flag
    #[serde(rename = "boolean")]
    Boolean,

    /// List/array of values
    #[serde(rename = "list")]
    List,

    /// Key-value mapping
    #[serde(rename = "object")]
    Object,

    /// Binary or structured data (JSON)
    #[serde(rename = "json")]
    Json,
}

impl VariableType {
    /// Check if this type can be directly serialized to string
    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            VariableType::String | VariableType::Number | VariableType::Boolean
        )
    }

    /// Get human-readable type name
    pub fn as_str(&self) -> &'static str {
        match self {
            VariableType::String => "string",
            VariableType::Number => "number",
            VariableType::Boolean => "boolean",
            VariableType::List => "list",
            VariableType::Object => "object",
            VariableType::Json => "json",
        }
    }
}
