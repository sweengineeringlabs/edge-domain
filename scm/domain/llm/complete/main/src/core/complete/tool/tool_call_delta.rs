//! Constructor and builder methods for [`ToolCallDelta`].

use crate::api::ToolCallDelta;

impl ToolCallDelta {
    /// Construct a tool-call delta at the given stream position.
    pub fn new(index: u32) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }

    /// Attach the tool id to this delta.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Attach the tool name to this delta.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(Self::normalized(name.into()));
        self
    }

    /// Attach a partial arguments string to this delta.
    pub fn with_arguments(mut self, arguments: impl Into<String>) -> Self {
        self.arguments = Some(arguments.into());
        self
    }

    /// Strip leading/trailing whitespace from a builder-supplied string.
    fn normalized(value: String) -> String {
        value.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_index() {
        let delta = ToolCallDelta::new(3);
        assert_eq!(delta.index, 3);
    }

    /// @covers: with_id
    #[test]
    fn test_with_id_sets_id() {
        let delta = ToolCallDelta::new(0).with_id("call-1");
        assert_eq!(delta.id, Some("call-1".to_string()));
    }

    /// @covers: with_name
    #[test]
    fn test_with_name_sets_name() {
        let delta = ToolCallDelta::new(0).with_name("get_weather");
        assert_eq!(delta.name, Some("get_weather".to_string()));
    }

    /// @covers: with_arguments
    #[test]
    fn test_with_arguments_sets_arguments() {
        let delta = ToolCallDelta::new(0).with_arguments("{}");
        assert_eq!(delta.arguments, Some("{}".to_string()));
    }

    /// @covers: normalized
    #[test]
    fn test_normalized_strips_whitespace() {
        assert_eq!(ToolCallDelta::normalized("  x  ".to_string()), "x");
    }
}
