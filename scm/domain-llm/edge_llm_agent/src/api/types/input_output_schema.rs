use serde_json::Value;

/// JSON Schema with examples and generation hints
#[derive(Debug, Clone)]
pub struct InputOutputSchema {
    pub schema: Value,
    pub description: String,
    pub examples: Vec<Value>,
}

impl InputOutputSchema {
    pub fn new(schema: Value, description: String) -> Self {
        Self {
            schema,
            description,
            examples: Vec::new(),
        }
    }
}
