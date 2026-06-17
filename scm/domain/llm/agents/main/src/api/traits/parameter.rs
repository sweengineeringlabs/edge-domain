//! Parameter — describes a skill parameter for discovery and documentation.

/// Describes a skill parameter for discovery and documentation.
#[derive(Debug, Clone)]
pub struct Parameter {
    /// The parameter name.
    pub name: String,
    /// Human-readable description of the parameter.
    pub description: String,
    /// The parameter's type (e.g. "string", "number", "object").
    pub param_type: String, // e.g., "string", "number", "object"
    /// Whether the parameter is required.
    pub required: bool,
}
