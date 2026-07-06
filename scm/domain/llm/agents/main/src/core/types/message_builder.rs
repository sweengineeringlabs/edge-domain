//! Constructors and fluent setters for [`MessageBuilder`].

use crate::api::{CacheControl, Message, MessageBuilder, MessageContent, Role, ToolCall};

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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_defaults_to_user_role() {
        assert_eq!(MessageBuilder::new().role, Role::User);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_message() {
        let msg = MessageBuilder::new().role(Role::Assistant).build();
        assert_eq!(msg.role, Role::Assistant);
    }

    /// @covers: role
    #[test]
    fn test_role_sets_role() {
        assert_eq!(MessageBuilder::new().role(Role::System).role, Role::System);
    }

    /// @covers: content
    #[test]
    fn test_content_sets_content() {
        let msg = MessageBuilder::new().content("hi").build();
        assert_eq!(msg.content, MessageContent::text("hi"));
    }

    /// @covers: name
    #[test]
    fn test_name_sets_name() {
        let msg = MessageBuilder::new().name("author").build();
        assert_eq!(msg.name, Some("author".to_string()));
    }

    /// @covers: tool_call_id
    #[test]
    fn test_tool_call_id_sets_id() {
        let msg = MessageBuilder::new().tool_call_id("call-1").build();
        assert_eq!(msg.tool_call_id, Some("call-1".to_string()));
    }

    /// @covers: tool_call
    #[test]
    fn test_tool_call_appends_one_call() {
        let call = ToolCall {
            id: "1".to_string(),
            name: "search".to_string(),
            arguments: "{}".to_string(),
        };
        let msg = MessageBuilder::new().tool_call(call).build();
        assert_eq!(msg.tool_calls.len(), 1);
    }

    /// @covers: tool_calls
    #[test]
    fn test_tool_calls_replaces_call_list() {
        let call = ToolCall {
            id: "1".to_string(),
            name: "search".to_string(),
            arguments: "{}".to_string(),
        };
        let msg = MessageBuilder::new()
            .tool_calls(vec![call.clone(), call])
            .build();
        assert_eq!(msg.tool_calls.len(), 2);
    }

    /// @covers: cache_control
    #[test]
    fn test_cache_control_sets_hint() {
        let msg = MessageBuilder::new()
            .cache_control(CacheControl::ephemeral())
            .build();
        assert_eq!(msg.cache_control, Some(CacheControl::ephemeral()));
    }
}
