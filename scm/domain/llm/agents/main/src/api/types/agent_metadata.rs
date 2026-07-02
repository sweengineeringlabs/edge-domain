//! Metadata about an agent — used for discovery and UI display.

use super::skill_metadata::SkillMetadata;

/// Metadata about an agent — used for discovery and UI display.
#[derive(Debug, Clone)]
pub struct AgentMetadata {
    /// Unique identifier of the agent.
    pub id: String,
    /// Human-readable name of the agent.
    pub name: String,
    /// Human-readable description of the agent's purpose.
    pub description: String,
    /// Version string of the agent.
    pub version: String,
    /// Skills the agent exposes.
    pub skills: Vec<SkillMetadata>,
    /// Reasoning patterns the agent supports (e.g. "react", "cot", "plan-execute").
    pub patterns: Vec<String>, // e.g., ["react", "cot", "plan-execute"]
}
