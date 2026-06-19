//! `Provider` — the LLM backend contract (primary trait).

use std::sync::Arc;

use edge_llm_complete::Completer;

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    FinishReason, ModelFamily, ModelInfo, ProviderConfig, TokenUsage, TokenizerAccuracy,
};

/// Pluggable LLM backend (OpenAI, Claude, local models, …).
///
/// Owns configuration, model metadata, and health state. Delegates all HTTP
/// completion work to an inner [`Completer`] returned by [`Provider::completer`].
pub trait Provider: Send + Sync {
    /// Stable identifier for this provider (e.g. `"anthropic"`).
    fn name(&self) -> &str;

    /// Configuration this provider was constructed with.
    fn provider_config(&self) -> ProviderConfig;

    /// Metadata for the active model.
    fn model_info(&self) -> ModelInfo;

    /// Model family the active model belongs to.
    fn model_family(&self) -> ModelFamily;

    /// Accuracy of this provider's token counting.
    fn tokenizer_accuracy(&self) -> TokenizerAccuracy;

    /// Token usage recorded by the most recent completion.
    fn last_token_usage(&self) -> TokenUsage;

    /// Finish reason recorded by the most recent completion.
    fn last_finish_reason(&self) -> FinishReason;

    /// Check whether the provider is reachable and configured.
    ///
    /// Returns [`ExecutionError::ProviderUnavailable`] when the backend cannot
    /// currently serve requests.
    fn health_check(&self) -> Result<(), ExecutionError>;

    /// The HTTP-level completion boundary this provider delegates to.
    ///
    /// Callers that need to issue a [`CompletionRequest`](edge_llm_complete::CompletionRequest)
    /// should call `provider.completer().complete(&request)` rather than calling the
    /// provider directly — completion is the completer's responsibility.
    fn completer(&self) -> Arc<dyn Completer>;
}
