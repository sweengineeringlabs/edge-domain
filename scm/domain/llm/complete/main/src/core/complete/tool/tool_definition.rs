//! Constructor for [`ToolDefinition`].

use serde_json::Value;

use crate::api::ToolDefinition;

impl ToolDefinition {
    /// Construct a tool definition.
    pub fn new(name: impl Into<String>, description: impl Into<String>, parameters: Value) -> Self {
        Self {
            name: Self::trimmed(name.into()),
            description: description.into(),
            parameters,
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
        let def = ToolDefinition::new("get_weather", "fetch weather", Value::Null);
        assert_eq!(def.name, "get_weather");
        assert_eq!(def.description, "fetch weather");
    }

    /// @covers: trimmed
    #[test]
    fn test_trimmed_strips_whitespace() {
        assert_eq!(
            ToolDefinition::trimmed("  get_weather  ".to_string()),
            "get_weather"
        );
    }
}
