//! Metadata about a skill — documents its interface and behavior.

/// Metadata about a skill — documents its interface and behavior.
#[derive(Debug, Clone)]
pub struct SkillMetadata {
    /// Human-readable name of the skill.
    pub name: String,
    /// Human-readable description of what the skill does.
    pub description: String,
    /// Serialized JSON Schema describing the skill's input, if any.
    pub input_schema: Option<String>, // JSON Schema (serialized)
    /// Serialized JSON Schema describing the skill's output, if any.
    pub output_schema: Option<String>, // JSON Schema (serialized)
    /// Whether the skill executes asynchronously.
    pub async_execution: bool,
    /// Whether the skill is expected to be long-running.
    pub long_running: bool,
}
