//! [`PromptMetadataResponse`] — response for [`Prompt::metadata`](crate::api::prompt::traits::Prompt::metadata).

use crate::api::prompt::types::Variable;

/// Metadata about this template (id, name, version, variables), flattened
/// from `PromptMetadata` (SEA `field_type_purity`: a freshly-computed value
/// with no borrow source cannot be nested by reference).
// @allow: suggest_builder_pattern — a Response DTO, always fully constructed
// in one shot by `Prompt::metadata`; never partially built by callers.
#[derive(Debug, PartialEq)]
pub struct PromptMetadataResponse {
    /// Unique identifier for this template.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Template version.
    pub version: String,
    /// Variables required by this template.
    pub variables: Vec<Variable>,
    /// Optional description/documentation.
    pub description: Option<String>,
    /// Estimated token count for template structure.
    pub base_token_count: u32,
    /// Tags for categorization.
    pub tags: Vec<String>,
}
