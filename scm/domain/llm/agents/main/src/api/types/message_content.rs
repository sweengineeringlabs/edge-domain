use super::ContentPart;
use serde::{Deserialize, Serialize};

/// Message content: plain text or structured multi-modal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    /// Plain text content.
    Text(String),
    /// Structured multi-modal content made up of individual parts.
    Parts(Vec<ContentPart>),
}

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
