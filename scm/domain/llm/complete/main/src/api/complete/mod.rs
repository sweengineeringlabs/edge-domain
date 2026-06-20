//! Complete domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::CompleteError;
pub use traits::{
    CacheableMessage, CompleteBootstrap, CompleteOps, CompleterHandler, Completer,
    CompletionStream, ContentFlattener, ModelOps, Processor, StreamOps, ToolOps, Validator,
};
pub use types::{
    CacheControl, CompletionRequest, CompletionResponse, ContentPart, EchoCompleter,
    FinishReason, ImageUrl, Message, MessageContent, ModelInfo, NoopCompleter, Role,
    StdCompleteFactory, StreamChunk, StreamDelta, ToolCall, ToolCallDelta, ToolChoice,
    ToolDefinition, TokenUsage,
};
