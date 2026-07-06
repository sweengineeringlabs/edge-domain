//! Constructors for [`Message`].

use crate::api::{Message, MessageContent, Role};

impl Message {
    /// Creates a message with the user role.
    pub fn user(content: impl Into<MessageContent>) -> Self {
        Self::with_role(Role::User, content)
    }

    /// Creates a message with the assistant role.
    pub fn assistant(content: impl Into<MessageContent>) -> Self {
        Self::with_role(Role::Assistant, content)
    }

    /// Creates a message with the system role.
    pub fn system(content: impl Into<MessageContent>) -> Self {
        Self::with_role(Role::System, content)
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

    /// Shared constructor for the role-specific factory methods.
    fn with_role(role: Role, content: impl Into<MessageContent>) -> Self {
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

    /// @covers: with_role
    #[test]
    fn test_with_role_sets_no_tool_call_id() {
        let msg = Message::with_role(Role::User, "hi");
        assert!(msg.tool_call_id.is_none());
    }
}
