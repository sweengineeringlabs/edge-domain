//! Complete domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::CompleteError;
pub use traits::{
    CacheableMessage, CompleteBootstrap, CompleteOps, Completer, CompleterHandler,
    CompletionStream, ContentFlattener, ModelOps, Processor, StreamOps, ToolOps, Validator,
};
pub use types::{
    CacheControl, CompletionRequest, CompletionResponse, ContentPart, EchoCompleter, FinishReason,
    Message, MessageContent, ModelInfo, NoopCompleter, StdCompleteFactory,
    StreamChunk, StreamDelta, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition,
};

// Internal API only (needed for trait signatures)
pub(super) use types::{ImageUrl, Role, TokenUsage};
