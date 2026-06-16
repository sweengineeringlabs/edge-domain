//! Parameter — describes a skill parameter for discovery and documentation.

/// Describes a skill parameter for discovery and documentation.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub param_type: String, // e.g., "string", "number", "object"
    pub required: bool,
}
