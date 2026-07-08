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

    /// Convert into an [`edge_llm_complete::ToolDefinition`] for a
    /// [`CompletionRequest`](edge_llm_complete::CompletionRequest).
    pub(crate) fn into_complete_tool(self) -> edge_llm_complete::ToolDefinition {
        edge_llm_complete::ToolDefinition {
            name: self.name,
            description: self.description,
            parameters: self.input_schema.into(),
        }
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

    /// @covers: into_complete_tool
    #[test]
    fn test_into_complete_tool_carries_name_and_description() {
        let tool = ToolDefinition::new("search", "search the web", crate::api::JsonValue::Null)
            .into_complete_tool();
        assert_eq!(tool.name, "search");
        assert_eq!(tool.description, "search the web");
    }
}
