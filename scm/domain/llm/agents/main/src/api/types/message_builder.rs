//! Builder for [`Message`] with a fluent API.

use crate::api::types::{CacheControl, Message, MessageContent, Role, ToolCall};

/// Builder for [`Message`] with fluent setters.
///
/// Defaults to an empty [`Role::User`] text message; setters override each
/// field before [`MessageBuilder::build`] assembles the final [`Message`].
#[derive(Debug, Clone)]
pub struct MessageBuilder {
    role: Role,
    content: MessageContent,
    name: Option<String>,
    tool_call_id: Option<String>,
    tool_calls: Vec<ToolCall>,
    cache_control: Option<CacheControl>,
}

impl MessageBuilder {
    /// Create a new builder defaulting to an empty user text message.
    pub fn new() -> Self {
        Self {
            role: Role::User,
            content: MessageContent::text(""),
            name: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
            cache_control: None,
        }
    }

    /// Set the conversation role.
    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    /// Set the message content.
    pub fn content(mut self, content: impl Into<MessageContent>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the optional author name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the tool-call identifier this message responds to.
    pub fn tool_call_id(mut self, id: impl Into<String>) -> Self {
        self.tool_call_id = Some(id.into());
        self
    }

    /// Append a tool call to the message.
    pub fn tool_call(mut self, call: ToolCall) -> Self {
        self.tool_calls.push(call);
        self
    }

    /// Replace the message's tool calls.
    pub fn tool_calls(mut self, calls: Vec<ToolCall>) -> Self {
        self.tool_calls = calls;
        self
    }

    /// Attach a prompt-caching hint.
    pub fn cache_control(mut self, cache_control: CacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Build the [`Message`].
    pub fn build(self) -> Message {
        Message {
            role: self.role,
            content: self.content,
            name: self.name,
            tool_call_id: self.tool_call_id,
            tool_calls: self.tool_calls,
            cache_control: self.cache_control,
        }
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
