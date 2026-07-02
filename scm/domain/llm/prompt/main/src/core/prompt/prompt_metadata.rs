//! Inherent methods for [`PromptMetadata`].

use crate::api::{PromptMetadata, Variable};

impl PromptMetadata {
    /// Create new template metadata
    pub fn new(id: String, name: String, version: String, variables: Vec<Variable>) -> Self {
        Self {
            id,
            name,
            version,
            variables,
            description: None,
            base_token_count: 0,
            tags: vec![],
        }
    }

    /// Get required (non-default) variables
    pub fn required_variables(&self) -> Vec<&Variable> {
        self.variables_where(true)
    }

    /// Get optional (has default) variables
    pub fn optional_variables(&self) -> Vec<&Variable> {
        self.variables_where(false)
    }

    /// Variables whose `required` flag matches `required`.
    fn variables_where(&self, required: bool) -> Vec<&Variable> {
        self.variables
            .iter()
            .filter(|v| v.required == required)
            .collect()
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Set base token count
    pub fn with_base_token_count(mut self, count: u32) -> Self {
        self.base_token_count = count;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::VariableKind;

    /// @covers: new
    #[test]
    fn test_new_sets_identity_fields() {
        let m = PromptMetadata::new("id".into(), "n".into(), "1".into(), vec![]);
        assert_eq!(m.id, "id");
        assert_eq!(m.base_token_count, 0);
    }

    /// @covers: required_variables
    #[test]
    fn test_required_variables_filters_required_only() {
        let var = Variable::new("a".into(), VariableKind::String);
        let m = PromptMetadata::new("id".into(), "n".into(), "1".into(), vec![var]);
        assert_eq!(m.required_variables().len(), 1);
    }

    /// @covers: optional_variables
    #[test]
    fn test_optional_variables_excludes_required() {
        let var = Variable::new("a".into(), VariableKind::String);
        let m = PromptMetadata::new("id".into(), "n".into(), "1".into(), vec![var]);
        assert!(m.optional_variables().is_empty());
    }

    /// @covers: with_tag
    #[test]
    fn test_with_tag_appends() {
        let m =
            PromptMetadata::new("id".into(), "n".into(), "1".into(), vec![]).with_tag("x".into());
        assert_eq!(m.tags, vec!["x".to_string()]);
    }

    /// @covers: with_base_token_count
    #[test]
    fn test_with_base_token_count_sets_value() {
        let m = PromptMetadata::new("id".into(), "n".into(), "1".into(), vec![])
            .with_base_token_count(7);
        assert_eq!(m.base_token_count, 7);
    }

    /// @covers: variables_where
    #[test]
    fn test_variables_where_matches_required_flag() {
        let var = Variable::new("a".into(), VariableKind::String);
        let m = PromptMetadata::new("id".into(), "n".into(), "1".into(), vec![var]);
        assert_eq!(m.variables_where(true).len(), 1);
        assert_eq!(m.variables_where(false).len(), 0);
    }
}
