//! `Provider` — the LLM backend contract (primary trait).

use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    CompletionInput, ExecutionStepResult, FinishReason, ModelFamily, ModelInfo, ProviderConfig,
    StreamChunk, TokenUsage, TokenizerAccuracy,
};

/// Pluggable LLM backend (OpenAI, Claude, local models, …).
///
/// Decouples agent orchestration from any specific backend: callers depend on
/// this contract and inject a concrete provider.
#[async_trait]
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

    /// Send a single-turn or multi-turn completion request to this backend.
    ///
    /// Returns [`ExecutionError::ProviderUnavailable`] on the noop reference
    /// implementation. Vendor backends override this with real HTTP dispatch.
    async fn complete(
        &self,
        input: &CompletionInput,
    ) -> Result<ExecutionStepResult, ExecutionError>;

    /// Initiate a streaming completion request to this backend.
    ///
    /// Returns an empty stream on the noop reference implementation. Vendor
    /// backends override this with a real HTTP streaming call.
    async fn stream(
        &self,
        input: &CompletionInput,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ExecutionError>>, ExecutionError>;
}
