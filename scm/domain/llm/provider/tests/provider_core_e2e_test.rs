//! Tests for the `ProviderCore` concrete provider implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_observer::StdObserveFactory;
use edge_llm_complete::NoopCompleter;
use edge_llm_provider::{
    FinishReason, ModelFamily, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory, TokenizerAccuracy,
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
        info,
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

/// @covers: ProviderCore::new — name reflects config model
#[test]
fn test_provider_core_new_name_matches_config_happy() {
    assert_eq!(make_provider("claude").name(), "claude");
}

/// @covers: ProviderCore::provider_config — returns config clone
#[test]
fn test_provider_core_provider_config_roundtrip_happy() {
    assert_eq!(make_provider("gpt").provider_config().model, "gpt");
}

/// @covers: ProviderCore::model_info — returns model metadata
#[test]
fn test_provider_core_model_info_name_matches_happy() {
    assert_eq!(make_provider("claude").model_info().name, "claude");
}

/// @covers: ProviderCore::model_family — family from metadata
#[test]
fn test_provider_core_model_family_returns_anthropic_happy() {
    assert_eq!(
        make_provider("claude").model_family(),
        ModelFamily::Anthropic
    );
}

/// @covers: ProviderCore::tokenizer_accuracy — always approximate
#[test]
fn test_provider_core_tokenizer_accuracy_approximate_happy() {
    assert_eq!(
        make_provider("claude").tokenizer_accuracy(),
        TokenizerAccuracy::Approximate
    );
}

/// @covers: ProviderCore::last_finish_reason — default stop
#[test]
fn test_provider_core_last_finish_reason_default_stop_happy() {
    assert_eq!(
        make_provider("claude").last_finish_reason(),
        FinishReason::Stop
    );
}

/// @covers: ProviderCore::health_check — ok when model set
#[test]
fn test_provider_core_health_check_ok_with_model_happy() {
    assert!(make_provider("claude").health_check().is_ok());
}

/// @covers: ProviderCore::health_check — error when model empty
#[test]
fn test_provider_core_health_check_errors_with_empty_model_error() {
    assert!(make_provider("").health_check().is_err());
}

/// @covers: ProviderCore::completer — returns the delegated completer
#[test]
fn test_provider_core_completer_returns_arc_happy() {
    let p = make_provider("claude");
    let c1 = p.completer();
    let c2 = p.completer();
    assert!(Arc::ptr_eq(&c1, &c2));
}
