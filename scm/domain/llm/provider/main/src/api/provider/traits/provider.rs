//! `Provider` — the LLM backend contract (primary trait).

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    FinishReason, ModelFamily, ModelInfo, ProviderConfig, TokenUsage, TokenizerAccuracy,
};

/// Pluggable LLM backend (OpenAI, Claude, local models, …).
///
/// Decouples agent orchestration from any specific backend: callers depend on
/// this contract and inject a concrete provider.
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
}
