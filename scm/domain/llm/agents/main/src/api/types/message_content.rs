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
