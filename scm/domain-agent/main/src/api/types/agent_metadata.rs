//! Metadata about an agent — used for discovery and UI display.

use super::skill_metadata::SkillMetadata;

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
