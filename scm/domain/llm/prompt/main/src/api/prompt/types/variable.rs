use crate::api::prompt::types::VariableType;
use serde::{Deserialize, Serialize};

/// A template variable with name, type, and optional default value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// Variable name (used in template as {{name}})
    pub name: String,

    /// Variable type
    pub var_type: VariableType,

    /// Current value (serde_json::Value for flexibility)
    pub value: Option<serde_json::Value>,

    /// Default value if not provided
    pub default: Option<serde_json::Value>,

    /// Whether this variable is required (no default)
    pub required: bool,

    /// Optional description for documentation
    pub description: Option<String>,
}

impl Variable {
    /// Create a new required variable (no default)
    pub fn new(name: String, var_type: VariableType) -> Self {
        Self {
            name,
            var_type,
            value: None,
            default: None,
            required: true,
            description: None,
        }
    }

    /// Create a variable with a default value
    pub fn with_default(name: String, var_type: VariableType, default: serde_json::Value) -> Self {
        Self {
            name,
            var_type,
            value: None,
            default: Some(default),
            required: false,
            description: None,
        }
    }

    /// Set the current value
    pub fn set_value(&mut self, value: serde_json::Value) {
        self.value = Some(value);
    }

    /// Get the effective value (current or default)
    pub fn get_value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref().or(self.default.as_ref())
    }

    /// Check if value is set or has default
    pub fn is_satisfied(&self) -> bool {
        self.value.is_some() || self.default.is_some()
    }

    /// Add description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
