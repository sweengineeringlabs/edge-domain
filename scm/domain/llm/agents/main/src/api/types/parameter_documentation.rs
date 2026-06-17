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

impl ParameterDocumentation {
    /// Creates a new parameter documentation entry with the given core fields.
    pub fn new(name: String, description: String, param_type: String, required: bool) -> Self {
        Self {
            name,
            description,
            param_type,
            required,
            default: None,
            examples: Vec::new(),
            validation_rules: None,
        }
    }
}
