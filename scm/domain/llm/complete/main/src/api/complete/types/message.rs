use serde::{Deserialize, Serialize};

use crate::api::complete::types::{CacheControl, MessageContent, Role, ToolCall};

/// A single turn in a conversation — the fundamental unit passed to a [`Completer`](crate::api::complete::traits::Completer).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Message {
    /// Author role of this message.
    pub role: Role,
    /// Message body (text or multi-modal parts).
    pub content: MessageContent,
    /// Optional participant name for multi-agent conversations.
    pub name: Option<String>,
    /// Tool result id — set on `Tool` role messages.
    pub tool_call_id: Option<String>,
    /// Tool calls requested by the model — set on `Assistant` role messages.
    pub tool_calls: Vec<ToolCall>,
    /// Anthropic prompt-caching hint.
    pub cache_control: Option<CacheControl>,
}

impl Message {
    /// Construct a user message with plain-text content.
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: Role::User, content: MessageContent::Text(content.into()), ..Default::default() }
    }

    /// Construct an assistant message with plain-text content.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: MessageContent::Text(content.into()),
            ..Default::default()
        }
    }

    /// Construct a system message with plain-text content.
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: Role::System, content: MessageContent::Text(content.into()), ..Default::default() }
    }

    /// Construct a tool-result message.
    pub fn tool(content: impl Into<String>, tool_call_id: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: MessageContent::Text(content.into()),
            tool_call_id: Some(tool_call_id.into()),
            ..Default::default()
        }
    }
}
