//! `Provider` implementation for [`ProviderCore`].

use std::fmt;
use std::sync::Arc;

use edge_llm_complete::Completer;

use crate::api::{
    ExecutionError, FinishReason, ModelFamily, ModelInfo, Provider, ProviderConfig, ProviderCore,
    TokenUsage, TokenizerAccuracy,
};

impl fmt::Debug for ProviderCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderCore")
            .field("config", &self.config)
            .field("model", &self.model)
            .field("completer", &"Arc<dyn Completer>")
            .finish()
    }
}

impl Clone for ProviderCore {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            model: self.model.clone(),
            completer: Arc::clone(&self.completer),
            observer: Arc::clone(&self.observer),
        }
    }
}

impl Provider for ProviderCore {
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

    fn completer(&self) -> Arc<dyn Completer> {
        Arc::clone(&self.completer)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use edge_domain_observer::StdObserveFactory;
    use edge_llm_complete::NoopCompleter;

    use crate::api::{ModelFamily, ModelInfo, Provider, ProviderConfig, ProviderCore};

    fn make_core(model: &str) -> ProviderCore {
        let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
        let info = ModelInfo::new(
            model.to_string(),
            model.to_string(),
            ModelFamily::Anthropic,
            8192,
        );
        ProviderCore::new(
            config,
            info,
            Arc::new(NoopCompleter),
            StdObserveFactory::noop_arc_observe_context(),
        )
    }

    #[test]
    fn test_new_constructs_with_model_name() {
        let core = make_core("claude");
        assert_eq!(core.name(), "claude");
    }

    #[test]
    fn test_health_check_ok_when_model_present() {
        assert!(make_core("claude").health_check().is_ok());
    }

    #[test]
    fn test_health_check_errors_when_model_empty() {
        assert!(make_core("").health_check().is_err());
    }

    #[test]
    fn test_completer_returns_arc_clone() {
        let core = make_core("claude");
        let c1 = core.completer();
        let c2 = core.completer();
        // Both arcs point at the same allocation — pointer equality.
        assert!(Arc::ptr_eq(&c1, &c2));
    }

    #[test]
    fn test_clone_produces_independent_value() {
        let original = make_core("claude");
        let cloned = original.clone();
        assert_eq!(original.name(), cloned.name());
    }

    #[test]
    fn test_model_family_reflects_metadata() {
        assert_eq!(make_core("claude").model_family(), ModelFamily::Anthropic);
    }

    #[test]
    fn test_debug_does_not_expose_completer_internals() {
        let s = format!("{:?}", make_core("claude"));
        assert!(s.contains("ProviderCore"));
        assert!(!s.contains("NoopCompleter"));
    }
}
