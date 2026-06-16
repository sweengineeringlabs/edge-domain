//! Metadata about a skill — documents its interface and behavior.

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
