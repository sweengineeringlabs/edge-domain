//! Constructors for [`StreamDelta`].

use crate::api::StreamDelta;

impl StreamDelta {
    /// Construct a text-only delta.
    pub fn text(content: impl Into<String>) -> Self {
        Self::build(Some(content.into()))
    }

    /// Construct an empty delta (heartbeat / keep-alive).
    pub fn empty() -> Self {
        Self::build(None)
    }

    /// Shared constructor for the `text`/`empty` factory methods.
    fn build(content: Option<String>) -> Self {
        Self {
            content,
            tool_calls: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: text
    #[test]
    fn test_text_sets_content() {
        let delta = StreamDelta::text("hi");
        assert_eq!(delta.content, Some("hi".to_string()));
    }

    /// @covers: empty
    #[test]
    fn test_empty_has_no_content() {
        let delta = StreamDelta::empty();
        assert_eq!(delta.content, None);
    }

    /// @covers: build
    #[test]
    fn test_build_sets_content_field() {
        let delta = StreamDelta::build(Some("hi".to_string()));
        assert_eq!(delta.content, Some("hi".to_string()));
        assert!(delta.tool_calls.is_none());
    }
}
