use serde_json::Value;

/// JSON Schema with examples and generation hints
#[derive(Debug, Clone)]
pub struct InputOutputSchema {
    /// The JSON Schema definition.
    pub schema: Value,
    /// Human-readable description of the schema.
    pub description: String,
    /// Example values conforming to the schema.
    pub examples: Vec<Value>,
}
