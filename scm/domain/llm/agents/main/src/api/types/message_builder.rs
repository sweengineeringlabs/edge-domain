//! Builder for [`Message`] with a fluent API.

use crate::api::types::{CacheControl, MessageContent, Role, ToolCall};

/// Builder for [`Message`] with fluent setters.
///
/// Defaults to an empty [`Role::User`] text message; setters override each
/// field before [`MessageBuilder::build`] assembles the final [`Message`].
#[derive(Debug, Clone)]
pub struct MessageBuilder {
    pub(crate) role: Role,
    pub(crate) content: MessageContent,
    pub(crate) name: Option<String>,
    pub(crate) tool_call_id: Option<String>,
    pub(crate) tool_calls: Vec<ToolCall>,
    pub(crate) cache_control: Option<CacheControl>,
}
