use crate::api::prompt::types::Variable;
use serde::{Deserialize, Serialize};

/// Metadata about a prompt template
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
