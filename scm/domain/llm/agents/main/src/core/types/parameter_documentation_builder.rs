//! Constructors and fluent setters for [`ParameterDocumentationBuilder`].

use serde_json::Value;

use crate::api::{ParameterDocumentation, ParameterDocumentationBuilder};

impl ParameterDocumentationBuilder {
    /// Create a builder from the required parameter fields.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        param_type: impl Into<String>,
        required: bool,
    ) -> Self {
        Self {
            name: Self::normalized(name.into()),
            description: description.into(),
            param_type: param_type.into(),
            required,
            default: None,
            examples: Vec::new(),
            validation_rules: None,
        }
    }

    /// Strip leading/trailing whitespace from a builder-supplied name.
    fn normalized(name: String) -> String {
        name.trim().to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_required_fields() {
        let builder = ParameterDocumentationBuilder::new("n", "d", "string", true);
        assert_eq!(builder.name, "n");
    }

    /// @covers: normalized
    #[test]
    fn test_normalized_strips_whitespace() {
        assert_eq!(
            ParameterDocumentationBuilder::normalized("  n  ".to_string()),
            "n"
        );
    }

    /// @covers: build
    #[test]
    fn test_build_produces_parameter_documentation() {
        let doc = ParameterDocumentationBuilder::new("n", "d", "string", true).build();
        assert_eq!(doc.name, "n");
    }

    /// @covers: default_value
    #[test]
    fn test_default_value_sets_default() {
        let doc = ParameterDocumentationBuilder::new("n", "d", "string", true)
            .default_value(Value::Bool(true))
            .build();
        assert_eq!(doc.default, Some(Value::Bool(true)));
    }

    /// @covers: example
    #[test]
    fn test_example_appends_one_example() {
        let doc = ParameterDocumentationBuilder::new("n", "d", "string", true)
            .example(Value::Bool(true))
            .build();
        assert_eq!(doc.examples.len(), 1);
    }

    /// @covers: examples
    #[test]
    fn test_examples_replaces_example_list() {
        let doc = ParameterDocumentationBuilder::new("n", "d", "string", true)
            .examples(vec![Value::Bool(true), Value::Bool(false)])
            .build();
        assert_eq!(doc.examples.len(), 2);
    }

    /// @covers: validation_rules
    #[test]
    fn test_validation_rules_sets_rules() {
        let doc = ParameterDocumentationBuilder::new("n", "d", "string", true)
            .validation_rules("min:0")
            .build();
        assert_eq!(doc.validation_rules, Some("min:0".to_string()));
    }
}
