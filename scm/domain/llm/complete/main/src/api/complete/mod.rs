//! Complete domain contracts: traits, value types, and errors.

pub(crate) mod completion;
pub mod errors;
pub(crate) mod stream;
pub(crate) mod tool;
pub mod traits;
pub mod types;

pub use errors::CompleteError;
pub use traits::{
    CacheableMessage, CompleteBootstrap, CompleteOps, Completer, CompleterHandler,
    CompletionStream, ContentFlattener, ModelOps, Processor, StreamOps, ToolOps, Validator,
};
pub use types::{
    AvailableToolsRequest, AvailableToolsResponse, CacheControlRequest, CacheControlResponse,
    CompleteBootstrapNameRequest, CompleteBootstrapNameResponse, CompleteRequest,
    CompleterHealthCheckRequest, CompleterHealthCheckResponse, CompletionCheckRequest,
    CompletionStreamRequest, CompletionStreamResponse, DeltaApplicationRequest, DeltaMergeRequest,
    FlattenRequest, FlattenResponse, ListModelsRequest, ListModelsResponse, MarkEphemeralRequest,
    ModelAvailabilityRequest, ModelAvailabilityResponse, ModelInfoRequest, ModelInfoResponse,
    ModelSupportRequest, ModelSupportResponse, ProcessingRequest, SupportedModelsRequest,
    SupportedModelsResponse, ToolChoicePreferenceRequest, ToolChoicePreferenceResponse,
    ToolExecutionRequest, ToolExecutionResponse, ValidationRequest,
};
pub use types::{
    CacheControl, CompletionRequest, CompletionResponse, ContentPart, EchoCompleter, FinishReason,
    ImageUrl, Message, MessageContent, ModelInfo, NoopCompleter, Role, StdCompleteFactory,
    StreamChunk, StreamDelta, TokenUsage, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition,
};
