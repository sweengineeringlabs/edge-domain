use serde::{Deserialize, Serialize};
use super::{CacheControl, MessageContent, Role, ToolCall};

/// A single message in a conversation turn
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

impl Message {
    pub fn user(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::Assistant, content)
    }

    pub fn system(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::System, content)
    }

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

    #[test]
    fn test_message_user_sets_user_role() {
        let msg = Message::user("hello");
        assert_eq!(msg.role, Role::User);
    }

    #[test]
    fn test_message_assistant_sets_assistant_role() {
        let msg = Message::assistant("response");
        assert_eq!(msg.role, Role::Assistant);
    }

    #[test]
    fn test_message_system_sets_system_role() {
        let msg = Message::system("you are helpful");
        assert_eq!(msg.role, Role::System);
    }

    #[test]
    fn test_message_tool_sets_tool_call_id() {
        let msg = Message::tool("result", "call-123");
        assert_eq!(msg.tool_call_id.as_deref(), Some("call-123"));
        assert_eq!(msg.role, Role::Tool);
    }
}
