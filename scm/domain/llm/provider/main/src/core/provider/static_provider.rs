//! `Provider` impl for `StaticProvider`.

use async_trait::async_trait;
use futures::stream::{self, BoxStream};

use crate::api::ExecutionError;
use crate::api::Provider;
use crate::api::{
    CompletionInput, ExecutionStepResult, FinishReason, ModelFamily, ModelInfo, ProviderConfig,
    StaticProvider, StreamChunk, TokenUsage, TokenizerAccuracy,
};

#[async_trait]
impl Provider for StaticProvider {
    fn name(&self) -> &str {
        &self.config.model
    }

    fn provider_config(&self) -> ProviderConfig {
        self.config.clone()
    }

    fn model_info(&self) -> ModelInfo {
        self.model.clone().unwrap_or_default()
    }

    fn model_family(&self) -> ModelFamily {
        self.model.as_ref().map(|m| m.family).unwrap_or_default()
    }

    fn tokenizer_accuracy(&self) -> TokenizerAccuracy {
        TokenizerAccuracy::Approximate
    }

    fn last_token_usage(&self) -> TokenUsage {
        TokenUsage::new(0, 0, 0, 0)
    }

    fn last_finish_reason(&self) -> FinishReason {
        FinishReason::Stop
    }

    fn health_check(&self) -> Result<(), ExecutionError> {
        if self.config.model.is_empty() {
            return Err(ExecutionError::ProviderUnavailable {
                message: "provider has no model configured".to_string(),
            });
        }
        Ok(())
    }

    async fn complete(
        &self,
        _input: &CompletionInput,
    ) -> Result<ExecutionStepResult, ExecutionError> {
        Ok(ExecutionStepResult::new(String::new(), None, 0.0, None))
    }

    async fn stream(
        &self,
        _input: &CompletionInput,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ExecutionError>>, ExecutionError> {
        Ok(Box::pin(stream::empty()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider(model: &str) -> StaticProvider {
        let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
        let info = ModelInfo::new(
            model.to_string(),
            model.to_string(),
            ModelFamily::Anthropic,
            8192,
        );
        StaticProvider::new(config, info)
    }

    #[test]
    fn test_name_returns_configured_model() {
        assert_eq!(provider("claude").name(), "claude");
    }

    #[test]
    fn test_model_family_reflects_metadata() {
        assert_eq!(provider("claude").model_family(), ModelFamily::Anthropic);
    }

    #[test]
    fn test_health_check_ok_when_model_present() {
        assert!(provider("claude").health_check().is_ok());
    }

    #[test]
    fn test_health_check_errors_when_model_empty() {
        assert!(provider("").health_check().is_err());
    }
}
