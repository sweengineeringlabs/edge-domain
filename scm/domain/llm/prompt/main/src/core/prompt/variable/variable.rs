//! Inherent methods for [`Variable`].

use crate::api::{JsonValue, Variable, VariableKind};

impl Variable {
    /// Create a new required variable (no default)
    pub fn new(name: String, var_type: VariableKind) -> Self {
        Self::with_fields(name, var_type, None, true)
    }

    /// Create a variable with a default value
    pub fn with_default(
        name: String,
        var_type: VariableKind,
        default: impl Into<JsonValue>,
    ) -> Self {
        Self::with_fields(name, var_type, Some(default.into()), false)
    }

    /// Shared field assembly for the public constructors above.
    fn with_fields(
        name: String,
        var_type: VariableKind,
        default: Option<JsonValue>,
        required: bool,
    ) -> Self {
        Self {
            name,
            var_type,
            value: None,
            default,
            required,
            description: None,
        }
    }

    /// Set the current value
    pub fn set_value(&mut self, value: impl Into<JsonValue>) {
        self.value = Some(value.into());
    }

    /// Get the effective value (current or default)
    pub fn get_value(&self) -> Option<&JsonValue> {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_defaults_to_required_no_value() {
        let v = Variable::new("a".into(), VariableKind::String);
        assert!(v.required);
        assert!(v.value.is_none());
    }

    /// @covers: with_default
    #[test]
    fn test_with_default_marks_optional() {
        let v = Variable::with_default("a".into(), VariableKind::String, JsonValue::Null);
        assert!(!v.required);
    }

    /// @covers: set_value
    #[test]
    fn test_set_value_stores_value() {
        let mut v = Variable::new("a".into(), VariableKind::String);
        v.set_value(JsonValue::String("x".into()));
        assert_eq!(v.value, Some(JsonValue::String("x".into())));
    }

    /// @covers: get_value
    #[test]
    fn test_get_value_prefers_value_over_default() {
        let mut v = Variable::with_default(
            "a".into(),
            VariableKind::String,
            JsonValue::String("d".into()),
        );
        v.set_value(JsonValue::String("v".into()));
        assert_eq!(v.get_value(), Some(&JsonValue::String("v".into())));
    }

    /// @covers: is_satisfied
    #[test]
    fn test_is_satisfied_false_when_neither_set() {
        let v = Variable::new("a".into(), VariableKind::String);
        assert!(!v.is_satisfied());
    }

    /// @covers: with_description
    #[test]
    fn test_with_description_sets_field() {
        let v = Variable::new("a".into(), VariableKind::String).with_description("d".into());
        assert_eq!(v.description, Some("d".to_string()));
    }

    /// @covers: with_fields
    #[test]
    fn test_with_fields_assembles_given_values() {
        let v = Variable::with_fields(
            "a".into(),
            VariableKind::Number,
            Some(JsonValue::Number(1.0)),
            false,
        );
        assert_eq!(v.var_type, VariableKind::Number);
        assert!(!v.required);
        assert_eq!(v.default, Some(JsonValue::Number(1.0)));
    }
}
