//! Inherent methods for [`RenderContext`].

use crate::api::{JsonValue, RenderContext};

impl RenderContext {
    /// Create a new empty render context
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a variable to context
    pub fn with_variable(mut self, name: String, value: impl Into<JsonValue>) -> Self {
        self.variables.insert(name, value.into());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set template ID
    pub fn with_template_id(mut self, id: String) -> Self {
        self.template_id = Some(id);
        self
    }

    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Option<&JsonValue> {
        self.variables.get(name)
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|v| v.as_str())
    }

    /// Check if all required variables are present
    pub fn has_all_variables(&self, required: &[&str]) -> bool {
        required.iter().all(|name| self.has_variable(name))
    }

    /// Whether a variable with the given name is present.
    fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Get count of set variables
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_empty() {
        assert_eq!(RenderContext::new().variable_count(), 0);
    }

    /// @covers: with_variable
    #[test]
    fn test_with_variable_accepts_raw_json_value() {
        let ctx = RenderContext::new().with_variable("a".into(), serde_json::json!("x"));
        assert_eq!(ctx.get_variable("a"), Some(&JsonValue::String("x".into())));
    }

    /// @covers: with_metadata
    #[test]
    fn test_with_metadata_stores_key_value() {
        let ctx = RenderContext::new().with_metadata("k".into(), "v".into());
        assert_eq!(ctx.get_metadata("k"), Some("v"));
    }

    /// @covers: with_template_id
    #[test]
    fn test_with_template_id_sets_field() {
        let ctx = RenderContext::new().with_template_id("t".into());
        assert_eq!(ctx.template_id, Some("t".to_string()));
    }

    /// @covers: has_all_variables
    #[test]
    fn test_has_all_variables_false_when_missing() {
        let ctx = RenderContext::new().with_variable("a".into(), serde_json::json!(1));
        assert!(!ctx.has_all_variables(&["a", "b"]));
    }

    /// @covers: variable_count
    #[test]
    fn test_variable_count_reflects_inserted() {
        let ctx = RenderContext::new().with_variable("a".into(), serde_json::json!(1));
        assert_eq!(ctx.variable_count(), 1);
    }

    /// @covers: has_variable
    #[test]
    fn test_has_variable_true_after_insert() {
        let ctx = RenderContext::new().with_variable("a".into(), serde_json::json!(1));
        assert!(ctx.has_variable("a"));
        assert!(!ctx.has_variable("b"));
    }

    /// @covers: get_variable
    #[test]
    fn test_get_variable_returns_inserted_value() {
        let ctx = RenderContext::new().with_variable("a".into(), serde_json::json!("x"));
        assert_eq!(ctx.get_variable("a"), Some(&JsonValue::String("x".into())));
        assert_eq!(ctx.get_variable("missing"), None);
    }

    /// @covers: get_metadata
    #[test]
    fn test_get_metadata_returns_inserted_value() {
        let ctx = RenderContext::new().with_metadata("k".into(), "v".into());
        assert_eq!(ctx.get_metadata("k"), Some("v"));
        assert_eq!(ctx.get_metadata("missing"), None);
    }
}
