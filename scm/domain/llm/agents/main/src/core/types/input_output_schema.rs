//! Constructor for [`InputOutputSchema`].

use serde_json::Value;

use crate::api::InputOutputSchema;

impl InputOutputSchema {
    /// Creates a new schema with the given definition and description.
    pub fn new(schema: Value, description: String) -> Self {
        Self {
            schema,
            description,
            examples: Vec::new(),
        }
    }
}

impl Default for InputOutputSchema {
    fn default() -> Self {
        Self::new(Value::Null, String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_has_no_examples() {
        let schema = InputOutputSchema::new(Value::Null, "desc".to_string());
        assert!(schema.examples.is_empty());
    }

    /// @covers: default
    #[test]
    fn test_default_has_null_schema() {
        assert_eq!(InputOutputSchema::default().schema, Value::Null);
    }
}
