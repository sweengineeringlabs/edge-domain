//! Agent and Skill metadata types.

use serde::{Deserialize, Serialize};

/// Metadata about an agent — used for discovery and UI display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub skills: Vec<SkillMetadata>,
    pub patterns: Vec<String>, // e.g., ["react", "cot", "plan-execute"]
}

/// Metadata about a skill — documents its interface and behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub input_schema: Option<serde_json::Value>, // JSON Schema
    pub output_schema: Option<serde_json::Value>, // JSON Schema
    pub async_execution: bool,
    pub long_running: bool,
}
