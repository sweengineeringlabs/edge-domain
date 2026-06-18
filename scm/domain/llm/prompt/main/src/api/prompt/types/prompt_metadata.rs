use crate::api::prompt::types::Variable;
use serde::{Deserialize, Serialize};

/// Metadata about a prompt template
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PromptMetadata {
    /// Unique identifier for this template
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Template version
    pub version: String,

    /// List of variables required by this template
    pub variables: Vec<Variable>,

    /// Optional description/documentation
    pub description: Option<String>,

    /// Estimated token count for template structure (before variable substitution)
    pub base_token_count: u32,

    /// Tags for categorization (e.g., "system", "user", "assistant")
    pub tags: Vec<String>,
}

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
        self.variables.iter().filter(|v| v.required).collect()
    }

    /// Get optional (has default) variables
    pub fn optional_variables(&self) -> Vec<&Variable> {
        self.variables.iter().filter(|v| !v.required).collect()
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
