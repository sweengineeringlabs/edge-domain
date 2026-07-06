use crate::api::prompt::types::{JsonValue, VariableKind};
use serde::{Deserialize, Serialize};

/// A template variable with name, type, and optional default value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    /// Variable name (used in template as {{name}})
    pub name: String,

    /// Variable type
    pub var_type: VariableKind,

    /// Current value
    pub value: Option<JsonValue>,

    /// Default value if not provided
    pub default: Option<JsonValue>,

    /// Whether this variable is required (no default)
    pub required: bool,

    /// Optional description for documentation
    pub description: Option<String>,
}
