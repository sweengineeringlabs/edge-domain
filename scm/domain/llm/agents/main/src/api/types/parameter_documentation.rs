use serde_json::Value;

/// Rich parameter documentation for skill I/O
#[derive(Debug, Clone)]
pub struct ParameterDocumentation {
    /// The parameter name.
    pub name: String,
    /// Human-readable description of the parameter.
    pub description: String,
    /// The parameter's type (e.g. "string", "number", "object").
    pub param_type: String,
    /// Whether the parameter is required.
    pub required: bool,
    /// Default value for the parameter, if any.
    pub default: Option<Value>,
    /// Example values for the parameter.
    pub examples: Vec<Value>,
    /// Serialized validation rules for the parameter, if any.
    pub validation_rules: Option<String>,
}
