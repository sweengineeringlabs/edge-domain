use serde_json::Value;

/// Rich parameter documentation for skill I/O
#[derive(Debug, Clone)]
pub struct ParameterDocumentation {
    pub name: String,
    pub description: String,
    pub param_type: String,
    pub required: bool,
    pub default: Option<Value>,
    pub examples: Vec<Value>,
    pub validation_rules: Option<String>,
}

impl ParameterDocumentation {
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
