//! Builder for [`ParameterDocumentation`] with a fluent API.

use crate::api::types::ParameterDocumentation;
use serde_json::Value;

/// Builder for [`ParameterDocumentation`] with fluent setters.
///
/// The four required fields (`name`, `description`, `param_type`, `required`)
/// are supplied up front via [`ParameterDocumentationBuilder::new`]; the
/// optional fields default to empty/`None` until overridden.
#[derive(Debug, Clone)]
pub struct ParameterDocumentationBuilder {
    name: String,
    description: String,
    param_type: String,
    required: bool,
    default: Option<Value>,
    examples: Vec<Value>,
    validation_rules: Option<String>,
}

impl ParameterDocumentationBuilder {
    /// Create a builder from the required parameter fields.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        param_type: impl Into<String>,
        required: bool,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            param_type: param_type.into(),
            required,
            default: None,
            examples: Vec::new(),
            validation_rules: None,
        }
    }

    /// Set the default value for the parameter.
    pub fn default_value(mut self, value: Value) -> Self {
        self.default = Some(value);
        self
    }

    /// Append an example value.
    pub fn example(mut self, value: Value) -> Self {
        self.examples.push(value);
        self
    }

    /// Replace the example values.
    pub fn examples(mut self, values: Vec<Value>) -> Self {
        self.examples = values;
        self
    }

    /// Set free-form validation rules describing accepted values.
    pub fn validation_rules(mut self, rules: impl Into<String>) -> Self {
        self.validation_rules = Some(rules.into());
        self
    }

    /// Build the [`ParameterDocumentation`].
    pub fn build(self) -> ParameterDocumentation {
        ParameterDocumentation {
            name: self.name,
            description: self.description,
            param_type: self.param_type,
            required: self.required,
            default: self.default,
            examples: self.examples,
            validation_rules: self.validation_rules,
        }
    }
}
