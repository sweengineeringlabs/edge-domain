//! Constructor for [`ToolCall`].

use crate::api::ToolCall;

impl ToolCall {
    /// Construct a tool call.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        arguments: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: Self::normalized_name(name.into()),
            arguments: arguments.into(),
        }
    }

    /// Strip leading/trailing whitespace from a tool name.
    fn normalized_name(name: String) -> String {
        name.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let call = ToolCall::new("call-1", "get_weather", "{}");
        assert_eq!(call.id, "call-1");
        assert_eq!(call.name, "get_weather");
    }

    /// @covers: normalized_name
    #[test]
    fn test_normalized_name_strips_whitespace() {
        assert_eq!(
            ToolCall::normalized_name("  get_weather  ".to_string()),
            "get_weather"
        );
    }
}
