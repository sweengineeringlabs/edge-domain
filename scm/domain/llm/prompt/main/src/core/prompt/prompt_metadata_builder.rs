//! Inherent methods for [`PromptMetadataBuilder`].

use crate::api::{PromptMetadata, PromptMetadataBuilder, Variable};

impl PromptMetadataBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the template identifier.
    pub fn id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

    /// Set the human-readable name.
    pub fn name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    /// Set the template version.
    pub fn version(mut self, value: String) -> Self {
        self.version = value;
        self
    }

    /// Set the declared variables.
    pub fn variables(mut self, value: Vec<Variable>) -> Self {
        self.variables = value;
        self
    }

    /// Set the documentation description.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the base token count.
    pub fn base_token_count(mut self, value: u32) -> Self {
        self.base_token_count = value;
        self
    }

    /// Set the categorization tags.
    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = value;
        self
    }

    /// Build the [`PromptMetadata`].
    pub fn build(self) -> PromptMetadata {
        let mut metadata = PromptMetadata::new(self.id, self.name, self.version, self.variables);
        Self::apply_optional_fields(
            &mut metadata,
            self.description,
            self.base_token_count,
            self.tags,
        );
        metadata
    }

    /// Apply the builder's optional (non-constructor) fields onto `metadata`.
    fn apply_optional_fields(
        metadata: &mut PromptMetadata,
        description: Option<String>,
        base_token_count: u32,
        tags: Vec<String>,
    ) {
        metadata.description = description;
        metadata.base_token_count = base_token_count;
        metadata.tags = tags;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::VariableKind;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_id() {
        assert_eq!(PromptMetadataBuilder::new().id, "");
    }

    /// @covers: build
    #[test]
    fn test_build_applies_all_overrides() {
        let m = PromptMetadataBuilder::new()
            .id("i".into())
            .name("n".into())
            .version("1".into())
            .tags(vec!["t".into()])
            .build();
        assert_eq!(m.id, "i");
        assert_eq!(m.tags, vec!["t".to_string()]);
    }

    /// @covers: apply_optional_fields
    #[test]
    fn test_apply_optional_fields_sets_all_three_fields() {
        let mut m = PromptMetadata::new("i".into(), "n".into(), "1".into(), vec![]);
        PromptMetadataBuilder::apply_optional_fields(&mut m, Some("d".into()), 5, vec!["t".into()]);
        assert_eq!(m.description, Some("d".to_string()));
        assert_eq!(m.base_token_count, 5);
        assert_eq!(m.tags, vec!["t".to_string()]);
    }

    /// @covers: id
    #[test]
    fn test_id_sets_field() {
        assert_eq!(PromptMetadataBuilder::new().id("i".into()).build().id, "i");
    }

    /// @covers: name
    #[test]
    fn test_name_sets_field() {
        assert_eq!(
            PromptMetadataBuilder::new().name("n".into()).build().name,
            "n"
        );
    }

    /// @covers: version
    #[test]
    fn test_version_sets_field() {
        assert_eq!(
            PromptMetadataBuilder::new()
                .version("2".into())
                .build()
                .version,
            "2"
        );
    }

    /// @covers: variables
    #[test]
    fn test_variables_sets_field() {
        let var = Variable::new("a".into(), VariableKind::String);
        let m = PromptMetadataBuilder::new().variables(vec![var]).build();
        assert_eq!(m.variables.len(), 1);
    }

    /// @covers: description
    #[test]
    fn test_description_sets_field() {
        let m = PromptMetadataBuilder::new().description("d".into()).build();
        assert_eq!(m.description, Some("d".to_string()));
    }

    /// @covers: base_token_count
    #[test]
    fn test_base_token_count_sets_field() {
        assert_eq!(
            PromptMetadataBuilder::new()
                .base_token_count(9)
                .build()
                .base_token_count,
            9
        );
    }

    /// @covers: tags
    #[test]
    fn test_tags_sets_field() {
        let m = PromptMetadataBuilder::new().tags(vec!["x".into()]).build();
        assert_eq!(m.tags, vec!["x".to_string()]);
    }
}
