//! `Provider` — the LLM backend contract (primary trait).

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    CompleterRequest, CompleterResponse, HealthCheckRequest, LastFinishReasonRequest,
    LastFinishReasonResponse, LastTokenUsageRequest, LastTokenUsageResponse, ModelFamilyRequest,
    ModelFamilyResponse, ModelInfoLookupRequest, ModelInfoResponse, ProviderConfigLookupRequest,
    ProviderConfigResponse, ProviderNameRequest, ProviderNameResponse, TokenizerAccuracyRequest,
    TokenizerAccuracyResponse,
};

/// Pluggable LLM backend (OpenAI, Claude, local models, …).
///
/// Owns configuration, model metadata, and health state. Delegates all HTTP
/// completion work to an inner [`Completer`](edge_llm_complete::Completer) returned by
/// [`Provider::completer`].
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
    /// Callers that need to issue a [`CompletionRequest`](edge_llm_complete::CompletionRequest)
    /// should call `provider.completer().complete(&request)` rather than calling the
    /// provider directly — completion is the completer's responsibility.
    fn completer(&self, req: CompleterRequest) -> Result<CompleterResponse, ExecutionError>;
}
