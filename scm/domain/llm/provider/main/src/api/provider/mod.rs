//! Provider domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub(crate) mod completion;
pub(crate) mod execution;
pub(crate) mod std;
pub(crate) mod stream;
pub(crate) mod tool;

pub use errors::{ExecutionError, OauthTokenSourceError};
pub use traits::{ExecutionModel, OauthTokenSourceResolver, Provider, StreamHandler};
pub use types::{
    AccumulateRequest, BufferedStreamHandler, CompleterRequest, CompleterResponse, CompletionInput,
    CompletionMessage, EchoExecutionModel, EchoProviderCompleter, ExecutionConfig,
    ExecutionConfigLookupRequest, ExecutionConfigResponse, ExecutionMode,
    ExecutionModeLookupRequest, ExecutionModeResponse, ExecutionReadinessRequest,
    ExecutionStepResult, FinishReason, HealthCheckRequest, JsonValue, LastFinishReasonRequest,
    LastFinishReasonResponse, LastTokenUsageRequest, LastTokenUsageResponse, MessageRole,
    ModelFamily, ModelFamilyRequest, ModelFamilyResponse, ModelInfo, ModelInfoLookupRequest,
    ModelInfoResponse, NextChunkRequest, NextChunkResponse, PendingToolCallRequest,
    PendingToolCallResponse,
    ProviderConfig, ProviderConfigLookupRequest, ProviderConfigResponse, ProviderNameRequest,
    ProviderNameResponse, StdProvider, StdProviderFactory, StepExecutionRequest,
    StepExecutionResponse, StreamChunk, StreamDelta, TokenSourceFileRequest,
    TokenSourceInitResponse, TokenUsage, TokenizerAccuracy, TokenizerAccuracyRequest,
    TokenizerAccuracyResponse, ToolCallDelta, ToolDefinition,
};
