//! Constructor for [`ToolDefinition`].

use crate::api::{JsonValue, ToolDefinition};

impl ToolDefinition {
    /// Construct a tool definition.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: impl Into<JsonValue>,
    ) -> Self {
        Self {
            name: Self::trimmed(name.into()),
            description: description.into(),
            input_schema: input_schema.into(),
        }
    }

    /// Strip leading/trailing whitespace from a tool name.
    fn trimmed(name: String) -> String {
        name.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let tool = ToolDefinition::new("search", "search the web", serde_json::json!({}));
        assert_eq!(tool.name, "search");
        assert_eq!(tool.description, "search the web");
    }

    /// @covers: trimmed
    #[test]
    fn test_trimmed_strips_whitespace() {
        assert_eq!(ToolDefinition::trimmed("  search  ".to_string()), "search");
    }
}
