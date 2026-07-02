//! `From` conversions into [`MessageContent`].

use crate::api::{ContentPart, MessageContent};

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}

impl From<Vec<ContentPart>> for MessageContent {
    fn from(parts: Vec<ContentPart>) -> Self {
        Self::Parts(parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_produces_text_variant() {
        let content: MessageContent = "hi".to_string().into();
        assert_eq!(content, MessageContent::Text("hi".to_string()));
    }

    #[test]
    fn test_from_str_produces_text_variant() {
        let content: MessageContent = "hi".into();
        assert_eq!(content, MessageContent::Text("hi".to_string()));
    }

    #[test]
    fn test_from_vec_content_part_produces_parts_variant() {
        let content: MessageContent = vec![ContentPart::text("hi")].into();
        assert!(matches!(content, MessageContent::Parts(_)));
    }
}
