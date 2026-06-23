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
    Message, MessageContent, ModelInfo, NoopCompleter, Role, StdCompleteFactory,
    StreamChunk, StreamDelta, TokenUsage, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition,
};
