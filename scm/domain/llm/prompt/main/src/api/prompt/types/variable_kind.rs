use serde::{Deserialize, Serialize};

/// Type of a prompt variable
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum VariableKind {
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
