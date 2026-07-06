//! Constructors and accessors for [`StreamDelta`].

use crate::api::{StreamDelta, ToolCallDelta};

impl StreamDelta {
    /// Create a text delta
    pub fn text(content: String) -> Self {
        Self::Text(content)
    }

    /// Create a tool call delta
    pub fn tool_calls(calls: Vec<ToolCallDelta>) -> Self {
        Self::ToolCalls(calls)
    }

    /// Create an empty delta
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Check if delta carries no content
    pub fn is_empty(&self) -> bool {
        self.is_empty_variant()
    }

    /// Whether this delta is the `Empty` variant.
    fn is_empty_variant(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: text
    #[test]
    fn test_text_creates_text_variant() {
        assert!(matches!(
            StreamDelta::text("hi".to_string()),
            StreamDelta::Text(_)
        ));
    }

    /// @covers: tool_calls
    #[test]
    fn test_tool_calls_creates_tool_calls_variant() {
        assert!(matches!(
            StreamDelta::tool_calls(vec![]),
            StreamDelta::ToolCalls(_)
        ));
    }

    /// @covers: empty
    #[test]
    fn test_empty_creates_empty_variant() {
        assert!(StreamDelta::empty().is_empty());
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_false_for_text() {
        assert!(!StreamDelta::text("x".to_string()).is_empty());
    }

    /// @covers: is_empty_variant
    #[test]
    fn test_is_empty_variant_true_for_empty() {
        assert!(StreamDelta::Empty.is_empty_variant());
    }
}
