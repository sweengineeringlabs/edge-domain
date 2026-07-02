//! Constructors and `From` conversions for [`MessageContent`].

use crate::api::{ContentPart, MessageContent};

impl MessageContent {
    /// Creates plain text message content.
    pub fn text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }

    /// Creates structured multi-modal message content from parts.
    pub fn parts(parts: Vec<ContentPart>) -> Self {
        Self::Parts(parts)
    }
}

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: text
    #[test]
    fn test_text_produces_text_variant() {
        assert_eq!(
            MessageContent::text("hi"),
            MessageContent::Text("hi".to_string())
        );
    }

    /// @covers: parts
    #[test]
    fn test_parts_produces_parts_variant() {
        let content = MessageContent::parts(vec![ContentPart::text("hi")]);
        assert!(matches!(content, MessageContent::Parts(_)));
    }

    /// @covers: from
    #[test]
    fn test_from_string_produces_text_variant() {
        let content: MessageContent = "hi".to_string().into();
        assert_eq!(content, MessageContent::Text("hi".to_string()));
    }

    /// @covers: from
    #[test]
    fn test_from_str_produces_text_variant() {
        let content: MessageContent = "hi".into();
        assert_eq!(content, MessageContent::Text("hi".to_string()));
    }
}
