//! `CompleteBootstrap` — constructor contract for the default complete primitives.

use serde_json::Value;

use crate::api::complete::types::{
    CacheControl, CompletionRequest, ContentPart, EchoCompleter, FinishReason, ImageUrl,
    Message, MessageContent, ModelInfo, NoopCompleter, Role, StdCompleteFactory, StreamChunk,
    StreamDelta, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition, TokenUsage,
};

/// Factory for the standard reference implementations and domain value constructors.
///
/// Implement on any unit struct to gain standard constructors. All methods have
/// default bodies so implementors need not override anything.
pub trait CompleteBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "complete"
    }

    /// Return a [`NoopCompleter`] that always returns [`CompleteError::ProviderNotFound`](crate::api::complete::errors::CompleteError::ProviderNotFound).
    fn noop_completer() -> NoopCompleter where Self: Sized {
        NoopCompleter
    }

    /// Return an [`EchoCompleter`] that reflects the last user message as content.
    fn echo_completer() -> EchoCompleter where Self: Sized {
        EchoCompleter
    }

    /// Build a [`CompletionRequest`] from a model id and message list.
    fn request(model: String, messages: Vec<Message>) -> CompletionRequest where Self: Sized {
        CompletionRequest::new(model, messages)
    }

    /// Build a [`Message`] with the given role and plain-text content.
    fn message(role: Role, content: String) -> Message where Self: Sized {
        Message { role, content: MessageContent::Text(content), ..Default::default() }
    }

    /// Build a user [`Message`].
    fn user_message(content: String) -> Message where Self: Sized {
        Message::user(content)
    }

    /// Build an assistant [`Message`].
    fn assistant_message(content: String) -> Message where Self: Sized {
        Message::assistant(content)
    }

    /// Build a system [`Message`].
    fn system_message(content: String) -> Message where Self: Sized {
        Message::system(content)
    }

    /// Build a tool-result [`Message`].
    fn tool_message(content: String, tool_call_id: String) -> Message where Self: Sized {
        Message::tool(content, tool_call_id)
    }

    /// Build a plain-text [`ContentPart`].
    fn text_part(text: String) -> ContentPart where Self: Sized {
        ContentPart::text(text)
    }

    /// Build an [`ImageUrl`] with no detail hint.
    fn image_url(url: String) -> ImageUrl where Self: Sized {
        ImageUrl::new(url)
    }

    /// Build an image-URL [`ContentPart`] from an [`ImageUrl`].
    fn image_part(image_url: ImageUrl) -> ContentPart where Self: Sized {
        ContentPart::image_url(image_url)
    }

    /// Build a [`ToolDefinition`].
    fn tool_definition(name: String, description: String, parameters: Value) -> ToolDefinition where Self: Sized {
        ToolDefinition::new(name, description, parameters)
    }

    /// Build a [`ToolCall`].
    fn tool_call(id: String, name: String, arguments: String) -> ToolCall where Self: Sized {
        ToolCall::new(id, name, arguments)
    }

    /// Build an initial [`ToolCallDelta`] at the given stream index.
    fn tool_call_delta(index: u32) -> ToolCallDelta where Self: Sized {
        ToolCallDelta::new(index)
    }

    /// Build a [`StreamDelta`] carrying text content.
    fn stream_delta(content: String) -> StreamDelta where Self: Sized {
        StreamDelta::text(content)
    }

    /// Build a terminal [`StreamChunk`] with a finish reason.
    fn stream_chunk(id: String, delta: StreamDelta, finish_reason: FinishReason) -> StreamChunk where Self: Sized {
        StreamChunk::terminal(id, delta, finish_reason)
    }

    /// Build a zeroed [`TokenUsage`].
    fn token_usage() -> TokenUsage where Self: Sized {
        TokenUsage::default()
    }

    /// Build a [`ModelInfo`] with all capability flags defaulting to false.
    fn model_info(
        id: String,
        name: String,
        provider: String,
        context_window: u32,
    ) -> ModelInfo where Self: Sized {
        ModelInfo::new(id, name, provider, context_window)
    }

    /// Build an ephemeral [`CacheControl`].
    fn cache_control() -> CacheControl where Self: Sized {
        CacheControl::ephemeral()
    }

    /// Build the `Auto` [`ToolChoice`].
    fn tool_choice_auto() -> ToolChoice where Self: Sized {
        ToolChoice::Auto
    }

    /// Return the standard factory implementation.
    fn std_complete_factory() -> StdCompleteFactory where Self: Sized {
        StdCompleteFactory
    }
}
