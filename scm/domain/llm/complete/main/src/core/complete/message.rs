//! Constructors and `CacheableMessage` impl for `Message`.

use crate::api::{
    CacheControl, CacheControlRequest, CacheControlResponse, CacheableMessage, CompleteError,
    MarkEphemeralRequest, Message, MessageContent, Role,
};

impl Message {
    /// Construct a user message with plain-text content.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Text(content.into()),
            ..Default::default()
        }
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
        Self {
            role: Role::System,
            content: MessageContent::Text(content.into()),
            ..Default::default()
        }
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

impl CacheableMessage for Message {
    fn with_cache_control(
        mut self,
        req: CacheControlRequest,
    ) -> Result<CacheControlResponse<Self>, CompleteError> {
        self.cache_control = Some(*req.cache);
        Ok(CacheControlResponse { message: self })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: user
    #[test]
    fn test_user_sets_user_role() {
        assert_eq!(Message::user("hi").role, Role::User);
    }

    /// @covers: assistant
    #[test]
    fn test_assistant_sets_assistant_role() {
        assert_eq!(Message::assistant("hi").role, Role::Assistant);
    }

    /// @covers: system
    #[test]
    fn test_system_sets_system_role() {
        assert_eq!(Message::system("hi").role, Role::System);
    }

    /// @covers: tool
    #[test]
    fn test_tool_sets_tool_call_id() {
        let msg = Message::tool("hi", "call-1");
        assert_eq!(msg.tool_call_id, Some("call-1".to_string()));
    }

    #[test]
    fn test_with_cache_control_sets_field() {
        let response = Message::user("hi")
            .with_cache_control(CacheControlRequest {
                cache: Box::new(CacheControl::ephemeral()),
            })
            .expect("with_cache_control ok");
        assert_eq!(
            response.message.cache_control,
            Some(CacheControl::ephemeral())
        );
    }

    #[test]
    fn test_mark_ephemeral_sets_ephemeral_cache_control() {
        let response = Message::user("hi")
            .mark_ephemeral(MarkEphemeralRequest)
            .expect("mark_ephemeral ok");
        assert_eq!(
            response.message.cache_control,
            Some(CacheControl::ephemeral())
        );
    }
}
