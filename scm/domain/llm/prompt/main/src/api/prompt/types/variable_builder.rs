//! `VariableBuilder` — fluent builder for [`Variable`].

use crate::api::prompt::types::{Variable, VariableType};

/// Fluent builder for [`Variable`].
#[derive(Clone, Debug)]
pub struct VariableBuilder {
    name: String,
    var_type: VariableType,
    value: Option<serde_json::Value>,
    default: Option<serde_json::Value>,
    required: bool,
    description: Option<String>,
}

impl Default for VariableBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            var_type: VariableType::String,
            value: None,
            default: None,
            required: true,
            description: None,
        }
    }
}

impl VariableBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the variable name.
    pub fn name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    /// Set the variable type.
    pub fn var_type(mut self, value: VariableType) -> Self {
        self.var_type = value;
        self
    }

    /// Set the current value.
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Set the default value (also marks the variable optional).
    pub fn default_value(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self.required = false;
        self
    }

    /// Set whether the variable is required.
    pub fn required(mut self, value: bool) -> Self {
        self.required = value;
        self
    }

    /// Set the documentation description.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Build the [`Variable`].
    pub fn build(self) -> Variable {
        Variable {
            name: self.name,
            var_type: self.var_type,
            value: self.value,
            default: self.default,
            required: self.required,
            description: self.description,
        }
    }
}
