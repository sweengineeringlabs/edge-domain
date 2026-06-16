//! Metadata about an agent — used for discovery and UI display.

use super::skill_metadata::SkillMetadata;
use crate::api::builder::AgentMetadataBuilder;

/// Metadata about an agent — used for discovery and UI display.
#[derive(Debug, Clone)]
pub struct AgentMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub skills: Vec<SkillMetadata>,
    pub patterns: Vec<String>, // e.g., ["react", "cot", "plan-execute"]
}

impl AgentMetadata {
    /// Create a new AgentMetadataBuilder for constructing AgentMetadata.
    pub fn builder() -> AgentMetadataBuilder {
        AgentMetadataBuilder::new()
    }
}
