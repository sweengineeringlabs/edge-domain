//! `Provider` — the LLM backend contract (primary trait).

use async_trait::async_trait;

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    CompleterRequest, CompleterResponse, HealthCheckRequest, LastFinishReasonRequest,
    LastFinishReasonResponse, LastTokenUsageRequest, LastTokenUsageResponse, ModelFamilyRequest,
    ModelFamilyResponse, ModelInfoLookupRequest, ModelInfoResponse, ProviderCompleteRequest,
    ProviderCompletionResponse, ProviderConfigLookupRequest, ProviderConfigResponse,
    ProviderNameRequest, ProviderNameResponse, TokenizerAccuracyRequest, TokenizerAccuracyResponse,
};

/// Pluggable LLM backend (OpenAI, Claude, local models, …).
///
/// Owns configuration, model metadata, and health state. Delegates all HTTP
/// completion work to an inner [`Completer`](edge_llm_complete::Completer) returned by
/// [`Provider::completer`].
#[async_trait]
pub trait Provider: Send + Sync {
    /// Stable identifier for this provider (e.g. `"anthropic"`).
    fn name(&self, req: ProviderNameRequest) -> Result<ProviderNameResponse, ExecutionError>;

    /// Configuration this provider was constructed with.
    fn provider_config(
        &self,
        req: ProviderConfigLookupRequest,
    ) -> Result<ProviderConfigResponse, ExecutionError>;

    /// Metadata for the active model.
    fn model_info(&self, req: ModelInfoLookupRequest) -> Result<ModelInfoResponse, ExecutionError>;

    /// Model family the active model belongs to.
    fn model_family(&self, req: ModelFamilyRequest) -> Result<ModelFamilyResponse, ExecutionError>;

    /// Accuracy of this provider's token counting.
    fn tokenizer_accuracy(
        &self,
        req: TokenizerAccuracyRequest,
    ) -> Result<TokenizerAccuracyResponse, ExecutionError>;

    /// Token usage recorded by the most recent completion.
    fn last_token_usage(
        &self,
        req: LastTokenUsageRequest,
    ) -> Result<LastTokenUsageResponse, ExecutionError>;

    /// Finish reason recorded by the most recent completion.
    fn last_finish_reason(
        &self,
        req: LastFinishReasonRequest,
    ) -> Result<LastFinishReasonResponse, ExecutionError>;

    /// Check whether the provider is reachable and configured.
    ///
    /// Returns [`ExecutionError::ProviderUnavailable`] when the backend cannot
    /// currently serve requests.
    fn health_check(&self, req: HealthCheckRequest) -> Result<(), ExecutionError>;

    /// The HTTP-level completion boundary this provider delegates to.
    ///
    /// Callers that need direct access to the [`Completer`](edge_llm_complete::Completer) —
    /// e.g. for streaming or model introspection — should use this rather than calling the
    /// provider directly. For a single structured completion, prefer [`Provider::complete`].
    fn completer(&self, req: CompleterRequest) -> Result<CompleterResponse, ExecutionError>;

    /// Run a structured completion through this provider's [`Completer`](edge_llm_complete::Completer).
    ///
    /// Fills in the model id from [`Provider::model_info`] and the sampling temperature from
    /// [`Provider::provider_config`], converts `req` into an
    /// [`edge_llm_complete::CompletionRequest`], and delegates to [`Provider::completer`].
    ///
    /// Returns [`ExecutionError`] when the underlying completer rejects or fails the request.
    async fn complete(
        &self,
        req: ProviderCompleteRequest,
    ) -> Result<ProviderCompletionResponse, ExecutionError>;
}
