//! `Provider` implementation for [`StdProvider`].

use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use edge_llm_complete::{CompleteRequest, Completer};

use crate::api::{
    CompleterRequest, CompleterResponse, ExecutionError, HealthCheckRequest,
    LastFinishReasonRequest, LastFinishReasonResponse, LastTokenUsageRequest,
    LastTokenUsageResponse, ModelFamilyRequest, ModelFamilyResponse, ModelInfo,
    ModelInfoLookupRequest, ModelInfoResponse, Provider, ProviderCompleteRequest,
    ProviderCompletionResponse, ProviderConfig, ProviderConfigLookupRequest,
    ProviderConfigResponse, ProviderNameRequest, ProviderNameResponse, StdProvider, TokenUsage,
    TokenizerAccuracyRequest, TokenizerAccuracyResponse,
};

impl StdProvider {
    /// Construct a provider core from config, model metadata, a completer delegate, and an observer.
    pub fn new(
        config: ProviderConfig,
        model: ModelInfo,
        completer: Arc<dyn Completer>,
        observer: Arc<dyn edge_domain_observer::ObserverContext>,
    ) -> Self {
        Self {
            config,
            model: Some(model),
            completer,
            observer,
        }
    }
}

impl fmt::Debug for StdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StdProvider")
            .field("config", &self.config)
            .field("model", &self.model)
            .field("completer", &"Arc<dyn Completer>")
            .finish()
    }
}

impl Clone for StdProvider {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            model: self.model.clone(),
            completer: Arc::clone(&self.completer),
            observer: Arc::clone(&self.observer),
        }
    }
}

#[async_trait]
impl Provider for StdProvider {
    fn name(&self, _req: ProviderNameRequest) -> Result<ProviderNameResponse, ExecutionError> {
        Ok(ProviderNameResponse {
            name: self.config.model.clone(),
        })
    }

    fn provider_config(
        &self,
        _req: ProviderConfigLookupRequest,
    ) -> Result<ProviderConfigResponse, ExecutionError> {
        Ok(ProviderConfigResponse {
            config: Box::new(self.config.clone()),
        })
    }

    fn model_info(
        &self,
        _req: ModelInfoLookupRequest,
    ) -> Result<ModelInfoResponse, ExecutionError> {
        Ok(ModelInfoResponse {
            info: Box::new(self.model.clone().unwrap_or_default()),
        })
    }

    fn model_family(
        &self,
        _req: ModelFamilyRequest,
    ) -> Result<ModelFamilyResponse, ExecutionError> {
        Ok(ModelFamilyResponse {
            family: self.model.as_ref().map(|m| m.family).unwrap_or_default(),
        })
    }

    fn tokenizer_accuracy(
        &self,
        _req: TokenizerAccuracyRequest,
    ) -> Result<TokenizerAccuracyResponse, ExecutionError> {
        Ok(TokenizerAccuracyResponse {
            accuracy: crate::api::TokenizerAccuracy::Approximate,
        })
    }

    fn last_token_usage(
        &self,
        _req: LastTokenUsageRequest,
    ) -> Result<LastTokenUsageResponse, ExecutionError> {
        Ok(LastTokenUsageResponse {
            usage: Box::new(TokenUsage::new(0, 0, 0, 0)),
        })
    }

    fn last_finish_reason(
        &self,
        _req: LastFinishReasonRequest,
    ) -> Result<LastFinishReasonResponse, ExecutionError> {
        Ok(LastFinishReasonResponse {
            reason: crate::api::FinishReason::Stop,
        })
    }

    fn health_check(&self, _req: HealthCheckRequest) -> Result<(), ExecutionError> {
        if self.config.model.is_empty() {
            return Err(ExecutionError::ProviderUnavailable {
                message: "provider has no model configured".to_string(),
            });
        }
        Ok(())
    }

    fn completer(&self, _req: CompleterRequest) -> Result<CompleterResponse, ExecutionError> {
        Ok(CompleterResponse {
            completer: Arc::clone(&self.completer),
        })
    }

    async fn complete(
        &self,
        req: ProviderCompleteRequest,
    ) -> Result<ProviderCompletionResponse, ExecutionError> {
        let model = self.model_info(ModelInfoLookupRequest)?.info.id.clone();
        let temperature = self
            .provider_config(ProviderConfigLookupRequest)?
            .config
            .temperature;
        let request = req.input.into_completion_request(model, temperature);

        self.completer
            .complete(CompleteRequest { request: &request })
            .await
            .map(|response| ProviderCompletionResponse { response })
            .map_err(ExecutionError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use edge_domain_observer::StdObserveFactory;
    use edge_llm_complete::NoopCompleter;

    use crate::api::{
        CompleterRequest, HealthCheckRequest, ModelFamily, ModelFamilyRequest, ModelInfo, Provider,
        ProviderConfig, ProviderNameRequest, StdProvider,
    };

    fn make_core(model: &str) -> StdProvider {
        let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
        let info = ModelInfo::new(
            model.to_string(),
            model.to_string(),
            ModelFamily::Anthropic,
            8192,
        );
        StdProvider::new(
            config,
            info,
            Arc::new(NoopCompleter),
            StdObserveFactory::noop_arc_observe_context(),
        )
    }

    #[test]
    fn test_new_constructs_with_model_name() {
        let core = make_core("claude");
        assert_eq!(
            core.name(ProviderNameRequest).expect("name ok").name,
            "claude"
        );
    }

    #[test]
    fn test_health_check_ok_when_model_present() {
        assert!(matches!(
            make_core("claude").health_check(HealthCheckRequest),
            Ok(())
        ));
    }

    #[test]
    fn test_health_check_errors_when_model_empty() {
        assert!(make_core("").health_check(HealthCheckRequest).is_err());
    }

    #[test]
    fn test_completer_returns_arc_clone() {
        let core = make_core("claude");
        let c1 = core
            .completer(CompleterRequest)
            .expect("completer ok")
            .completer;
        let c2 = core
            .completer(CompleterRequest)
            .expect("completer ok")
            .completer;
        // Both arcs point at the same allocation — pointer equality.
        assert!(Arc::ptr_eq(&c1, &c2));
    }

    #[test]
    fn test_clone_produces_independent_value() {
        let original = make_core("claude");
        let cloned = original.clone();
        assert_eq!(
            original.name(ProviderNameRequest).expect("name ok").name,
            cloned.name(ProviderNameRequest).expect("name ok").name
        );
    }

    #[test]
    fn test_model_family_reflects_metadata() {
        assert_eq!(
            make_core("claude")
                .model_family(ModelFamilyRequest)
                .expect("model_family ok")
                .family,
            ModelFamily::Anthropic
        );
    }

    #[test]
    fn test_debug_does_not_expose_completer_internals() {
        let s = format!("{:?}", make_core("claude"));
        assert!(s.contains("StdProvider"));
        assert!(!s.contains("NoopCompleter"));
    }
}
