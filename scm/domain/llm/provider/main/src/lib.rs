//! # edge-llm-provider
//!
//! LLM Provider domain primitive (ADR-033): a pluggable execution-backend
//! abstraction for swappable LLM providers (OpenAI, Claude, local models).
//!
//! Public surface is delegated entirely through `saf/`.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use api::{
    AccumulateRequest, BufferedStreamHandler, CompleterRequest, CompleterResponse, CompletionInput,
    CompletionMessage, EchoExecutionModel, EchoProviderCompleter, ExecutionConfig,
    ExecutionConfigLookupRequest, ExecutionConfigResponse, ExecutionError, ExecutionMode,
    ExecutionModeLookupRequest, ExecutionModeResponse, ExecutionReadinessRequest,
    ExecutionStepResult, FinishReason, HealthCheckRequest, JsonValue, LastFinishReasonRequest,
    LastFinishReasonResponse, LastTokenUsageRequest, LastTokenUsageResponse, MessageRole,
    ModelFamily, ModelFamilyRequest, ModelFamilyResponse, ModelInfo, ModelInfoLookupRequest,
    ModelInfoResponse, NextChunkRequest, NextChunkResponse, OauthTokenSourceError,
    PendingToolCallRequest, PendingToolCallResponse, ProviderConfig, ProviderConfigLookupRequest,
    ProviderConfigResponse, ProviderNameRequest, ProviderNameResponse, StdProvider,
    StdProviderFactory, StepExecutionRequest, StepExecutionResponse, StreamChunk, StreamDelta,
    TokenSourceFileRequest, TokenSourceInitResponse, TokenUsage, TokenizerAccuracy,
    TokenizerAccuracyRequest, TokenizerAccuracyResponse, ToolCallDelta, ToolDefinition,
};
pub use saf::{
    ExecutionModel, OauthTokenSourceResolver, Provider, StreamHandler, EXECUTION_MODEL_SVC,
    EXECUTION_MODEL_SVC_FACTORY, OAUTH_TOKEN_SOURCE_RESOLVER_SVC,
    OAUTH_TOKEN_SOURCE_RESOLVER_SVC_FACTORY, PROVIDER_COMPLETER_SVC, PROVIDER_SVC,
    PROVIDER_SVC_FACTORY, STREAM_HANDLER_SVC, STREAM_HANDLER_SVC_FACTORY,
};
