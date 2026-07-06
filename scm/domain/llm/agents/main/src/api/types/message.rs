use super::{CacheControl, MessageContent, Role, ToolCall};
use serde::{Deserialize, Serialize};

/// A single message in a conversation turn
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender.
    pub role: Role,
    /// The message content.
    pub content: MessageContent,
    /// Optional name of the sender (e.g. for tool/function attribution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier of the tool call this message responds to, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls requested by the assistant in this message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    /// Optional prompt-caching hint for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}
