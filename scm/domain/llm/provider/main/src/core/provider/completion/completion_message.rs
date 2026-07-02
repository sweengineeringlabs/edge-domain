//! Constructors for [`CompletionMessage`].

use crate::api::{CompletionMessage, MessageRole};

impl CompletionMessage {
    /// Construct a user-role message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::with_role(MessageRole::User, content)
    }

    /// Construct an assistant-role message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::with_role(MessageRole::Assistant, content)
    }

    /// Construct a tool-result message.
    pub fn tool(content: impl Into<String>) -> Self {
        Self::with_role(MessageRole::Tool, content)
    }

    /// Shared constructor for the role-specific factory methods.
    fn with_role(role: MessageRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: user
    #[test]
    fn test_user_sets_user_role() {
        assert_eq!(CompletionMessage::user("hi").role, MessageRole::User);
    }

    /// @covers: assistant
    #[test]
    fn test_assistant_sets_assistant_role() {
        assert_eq!(
            CompletionMessage::assistant("hi").role,
            MessageRole::Assistant
        );
    }

    /// @covers: tool
    #[test]
    fn test_tool_sets_tool_role() {
        assert_eq!(CompletionMessage::tool("hi").role, MessageRole::Tool);
    }

    /// @covers: with_role
    #[test]
    fn test_with_role_sets_content() {
        assert_eq!(
            CompletionMessage::with_role(MessageRole::User, "hi").content,
            "hi"
        );
    }
}
