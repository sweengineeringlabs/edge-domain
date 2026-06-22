use serde::{Deserialize, Serialize};

use crate::api::provider::types::MessageRole;

/// A single message in a multi-turn completion conversation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionMessage {
    /// Who produced this message.
    pub role: MessageRole,
    /// Text content of the message.
    pub content: String,
}

impl CompletionMessage {
    /// Construct a user-role message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
        }
    }

    /// Construct an assistant-role message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
        }
    }

    /// Construct a tool-result message.
    pub fn tool(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Tool,
            content: content.into(),
        }
    }
}
