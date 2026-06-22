//! # edge-llm-complete
//!
//! LLM Complete domain primitive (ADR-043): canonical HTTP-level completion
//! port contract — the boundary that provider plugin backends implement and
//! that agents and reasoning pipelines consume.
//!
//! Public surface is delegated entirely through `saf/`.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

// CompletionStream is a type alias; saf_no_trait_reexport forbids pub use *Stream
// in saf/ files, so it is re-exported here directly from api/.
pub use crate::api::CompletionStream;

pub use saf::{
    CacheControl, CacheableMessage, CompleteBootstrap, CompleteError, CompleteOps, Completer,
    CompleterHandler, CompletionRequest, CompletionResponse, ContentFlattener, ContentPart,
    EchoCompleter, FinishReason, ImageUrl, Message, MessageContent, ModelInfo, ModelOps,
    NoopCompleter, Processor, Role, StdCompleteFactory, StreamChunk, StreamDelta, StreamOps,
    TokenUsage, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition, ToolOps, Validator,
    CACHEABLE_MESSAGE_SVC, COMPLETER_HANDLER_SVC, COMPLETER_SVC, COMPLETE_FACTORY_SVC,
    COMPLETE_OPS_SVC, CONTENT_FLATTENER_SVC, MODEL_OPS_SVC, PROCESSOR_SVC, STREAM_OPS_SVC,
    TOOL_OPS_SVC, VALIDATOR_SVC,
};
