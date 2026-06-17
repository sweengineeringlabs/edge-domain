//! `PromptMetadataBuilder` — fluent builder for [`PromptMetadata`].

use crate::api::prompt::types::{PromptMetadata, Variable};

/// Fluent builder for [`PromptMetadata`].
#[derive(Clone, Debug, Default)]
pub struct PromptMetadataBuilder {
    id: String,
    name: String,
    version: String,
    variables: Vec<Variable>,
    description: Option<String>,
    base_token_count: u32,
    tags: Vec<String>,
}

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
        metadata.description = self.description;
        metadata.base_token_count = self.base_token_count;
        metadata.tags = self.tags;
        metadata
    }
}
