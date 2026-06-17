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

impl Message {
    /// Creates a message with the user role.
    pub fn user(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::User, content)
    }

    /// Creates a message with the assistant role.
    pub fn assistant(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::Assistant, content)
    }

    /// Creates a message with the system role.
    pub fn system(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::System, content)
    }

    /// Creates a tool-result message responding to the given tool call.
    pub fn tool(content: impl Into<MessageContent>, tool_call_id: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: content.into(),
            name: None,
            tool_call_id: Some(tool_call_id.into()),
            tool_calls: vec![],
            cache_control: None,
        }
    }

    fn new(role: Role, content: impl Into<MessageContent>) -> Self {
        Self {
            role,
            content: content.into(),
            name: None,
            tool_call_id: None,
            tool_calls: vec![],
            cache_control: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: user
    #[test]
    fn test_message_user_sets_user_role() {
        let msg = Message::user("hello");
        assert_eq!(msg.role, Role::User);
    }

    /// @covers: assistant
    #[test]
    fn test_message_assistant_sets_assistant_role() {
        let msg = Message::assistant("response");
        assert_eq!(msg.role, Role::Assistant);
    }

    /// @covers: system
    #[test]
    fn test_message_system_sets_system_role() {
        let msg = Message::system("you are helpful");
        assert_eq!(msg.role, Role::System);
    }

    /// @covers: tool
    #[test]
    fn test_message_tool_sets_tool_call_id() {
        let msg = Message::tool("result", "call-123");
        assert_eq!(msg.tool_call_id.as_deref(), Some("call-123"));
        assert_eq!(msg.role, Role::Tool);
    }
}
