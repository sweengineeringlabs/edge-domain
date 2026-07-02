//! Tests for the `StdProvider` concrete provider implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_observer::StdObserveFactory;
use edge_llm_complete::NoopCompleter;
use edge_llm_provider::{
    CompleterRequest, FinishReason, HealthCheckRequest, LastFinishReasonRequest, ModelFamily,
    ModelFamilyRequest, ModelInfo, ModelInfoLookupRequest, Provider, ProviderBootstrap,
    ProviderConfig, ProviderConfigLookupRequest, ProviderNameRequest, StdProviderFactory,
    TokenizerAccuracy, TokenizerAccuracyRequest,
};

fn make_provider(model: &str) -> Arc<dyn Provider> {
    let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        model.to_string(),
        model.to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    StdProviderFactory::provider(
        config,
        Box::new(info),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

/// @covers: StdProvider::new — name reflects config model
#[test]
fn test_std_provider_new_name_matches_config_happy() {
    assert_eq!(
        make_provider("claude")
            .name(ProviderNameRequest)
            .unwrap()
            .name,
        "claude"
    );
}

/// @covers: StdProvider::provider_config — returns config clone
#[test]
fn test_std_provider_provider_config_roundtrip_happy() {
    assert_eq!(
        make_provider("gpt")
            .provider_config(ProviderConfigLookupRequest)
            .unwrap()
            .config
            .model,
        "gpt"
    );
}

/// @covers: StdProvider::model_info — returns model metadata
#[test]
fn test_std_provider_model_info_name_matches_happy() {
    assert_eq!(
        make_provider("claude")
            .model_info(ModelInfoLookupRequest)
            .unwrap()
            .info
            .name,
        "claude"
    );
}

/// @covers: StdProvider::model_family — family from metadata
#[test]
fn test_std_provider_model_family_returns_anthropic_happy() {
    assert_eq!(
        make_provider("claude")
            .model_family(ModelFamilyRequest)
            .unwrap()
            .family,
        ModelFamily::Anthropic
    );
}

/// @covers: StdProvider::tokenizer_accuracy — always approximate
#[test]
fn test_std_provider_tokenizer_accuracy_approximate_happy() {
    assert_eq!(
        make_provider("claude")
            .tokenizer_accuracy(TokenizerAccuracyRequest)
            .unwrap()
            .accuracy,
        TokenizerAccuracy::Approximate
    );
}

/// @covers: StdProvider::last_finish_reason — default stop
#[test]
fn test_std_provider_last_finish_reason_default_stop_happy() {
    assert_eq!(
        make_provider("claude")
            .last_finish_reason(LastFinishReasonRequest)
            .unwrap()
            .reason,
        FinishReason::Stop
    );
}

/// @covers: StdProvider::health_check — ok when model set
#[test]
fn test_std_provider_health_check_ok_with_model_happy() {
    // `ExecutionError` does not implement `PartialEq`, so `Result<(), ExecutionError>`
    // cannot be compared with `assert_eq!`; match on the `Ok(())` payload instead.
    assert!(matches!(
        make_provider("claude").health_check(HealthCheckRequest),
        Ok(())
    ));
}

/// @covers: StdProvider::health_check — error when model empty
#[test]
fn test_std_provider_health_check_errors_with_empty_model_error() {
    assert!(make_provider("").health_check(HealthCheckRequest).is_err());
}

/// @covers: StdProvider::completer — returns the delegated completer
#[test]
fn test_std_provider_completer_returns_arc_happy() {
    let p = make_provider("claude");
    let c1 = p.completer(CompleterRequest).unwrap().completer;
    let c2 = p.completer(CompleterRequest).unwrap().completer;
    assert!(Arc::ptr_eq(&c1, &c2));
}
