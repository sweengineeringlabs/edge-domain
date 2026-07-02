//! `Default` and inherent methods for [`VariableBuilder`].

use crate::api::{JsonValue, Variable, VariableBuilder, VariableKind};

impl Default for VariableBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            var_type: VariableKind::String,
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
    pub fn var_type(mut self, value: VariableKind) -> Self {
        self.var_type = value;
        self
    }

    /// Set the current value.
    pub fn value(mut self, value: impl Into<JsonValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the default value (also marks the variable optional).
    pub fn default_value(mut self, value: impl Into<JsonValue>) -> Self {
        self.default = Some(value.into());
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_name() {
        assert_eq!(VariableBuilder::new().name, "");
    }

    /// @covers: default
    #[test]
    fn test_default_is_required_string_kind() {
        let b = VariableBuilder::default();
        assert!(b.required);
        assert_eq!(b.var_type, VariableKind::String);
    }

    /// @covers: build
    #[test]
    fn test_build_applies_all_overrides() {
        let v = VariableBuilder::new()
            .name("n".into())
            .var_type(VariableKind::Number)
            .default_value(JsonValue::Number(1.0))
            .build();
        assert_eq!(v.name, "n");
        assert!(!v.required);
    }

    /// @covers: name
    #[test]
    fn test_name_sets_field() {
        assert_eq!(VariableBuilder::new().name("n".into()).build().name, "n");
    }

    /// @covers: var_type
    #[test]
    fn test_var_type_sets_field() {
        let v = VariableBuilder::new()
            .var_type(VariableKind::Boolean)
            .build();
        assert_eq!(v.var_type, VariableKind::Boolean);
    }

    /// @covers: value
    #[test]
    fn test_value_sets_field() {
        let v = VariableBuilder::new().value(JsonValue::Bool(true)).build();
        assert_eq!(v.value, Some(JsonValue::Bool(true)));
    }

    /// @covers: default_value
    #[test]
    fn test_default_value_marks_optional() {
        let v = VariableBuilder::new()
            .default_value(JsonValue::Bool(true))
            .build();
        assert_eq!(v.default, Some(JsonValue::Bool(true)));
        assert!(!v.required);
    }

    /// @covers: required
    #[test]
    fn test_required_sets_field() {
        let v = VariableBuilder::new().required(false).build();
        assert!(!v.required);
    }

    /// @covers: description
    #[test]
    fn test_description_sets_field() {
        let v = VariableBuilder::new().description("d".into()).build();
        assert_eq!(v.description, Some("d".to_string()));
    }
}
