//! Metadata about a skill — documents its interface and behavior.

use crate::api::builder::SkillMetadataBuilder;

/// Metadata about a skill — documents its interface and behavior.
#[derive(Debug, Clone)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub input_schema: Option<String>, // JSON Schema (serialized)
    pub output_schema: Option<String>, // JSON Schema (serialized)
    pub async_execution: bool,
    pub long_running: bool,
}

impl SkillMetadata {
    /// Create a new SkillMetadataBuilder for constructing SkillMetadata.
    pub fn builder() -> SkillMetadataBuilder {
        SkillMetadataBuilder::new()
    }
}
