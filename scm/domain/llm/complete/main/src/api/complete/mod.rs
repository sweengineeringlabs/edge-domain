//! Complete domain contracts: traits, value types, and errors.

pub(crate) mod completion;
pub mod errors;
pub(crate) mod stream;
pub(crate) mod tool;
pub(crate) mod tool_call_batch;
pub(crate) mod tool_call_step;
pub mod traits;
pub mod types;

pub use errors::CompleteError;
pub use traits::{
    CacheableMessage, CompleteOps, Completer, CompleterHandler, CompletionStream,
    ContentFlattener, ModelOps, Processor, StreamOps, ToolCallLoop, ToolOps, ToolResultBatch,
    Validator,
};
pub use types::{
    AvailableToolsRequest, AvailableToolsResponse, CacheControlRequest, CacheControlResponse,
    CompleteRequest,
    CompleterHealthCheckRequest, CompleterHealthCheckResponse, CompletionCheckRequest,
    CompletionStreamRequest, CompletionStreamResponse, DeltaApplicationRequest, DeltaMergeRequest,
    FlattenRequest, FlattenResponse, ListModelsRequest, ListModelsResponse, MarkEphemeralRequest,
    ModelAvailabilityRequest, ModelAvailabilityResponse, ModelInfoRequest, ModelInfoResponse,
    ModelSupportRequest, ModelSupportResponse, ProcessingRequest, SupportedModelsRequest,
    SupportedModelsResponse, ToolCallLoopRequest, ToolCallLoopResponse,
    ToolChoicePreferenceRequest, ToolChoicePreferenceResponse, ToolExecutionRequest,
    ToolExecutionResponse, ToolRecordRequest, ValidationRequest,
};
pub use types::{
    BoundedToolCallLoop, CacheControl, CompletionRequest, CompletionResponse, ContentPart,
    EchoCompleter, FinishReason, ImageUrl, Message, MessageContent, ModelInfo, NoopCompleter, Role,
    StreamChunk, StreamDelta, TokenUsage, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition,
};
