//! Constructors and conversions for [`CompletionMessage`].

use edge_llm_complete::{Message, MessageContent};

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

    /// Convert into an [`edge_llm_complete::Message`] for a [`CompletionRequest`](edge_llm_complete::CompletionRequest).
    pub(crate) fn into_message(self) -> Message {
        Message {
            role: self.role.into_role(),
            content: MessageContent::Text(self.content),
            ..Message::default()
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

    /// @covers: into_message
    #[test]
    fn test_into_message_maps_role_and_text_content() {
        let message = CompletionMessage::user("hi").into_message();
        assert_eq!(message.role, edge_llm_complete::Role::User);
        assert_eq!(message.content, MessageContent::Text("hi".to_string()));
    }
}
