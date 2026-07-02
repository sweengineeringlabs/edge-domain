//! Layer-level e2e coverage for the `ProviderBootstrap` trait via a test-double implementer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_observer::StdObserveFactory;
use edge_llm_complete::NoopCompleter;
use edge_llm_provider::{
    ModelFamily, ModelInfo, Provider, ProviderBootstrap, ProviderBootstrapNameRequest,
    ProviderConfig, ProviderNameRequest,
};

struct BootstrapDouble;

impl ProviderBootstrap for BootstrapDouble {
    fn provider(
        config: ProviderConfig,
        model: Box<ModelInfo>,
        completer: Arc<dyn edge_llm_complete::Completer>,
        observer: Arc<dyn edge_domain_observer::ObserverContext>,
    ) -> Arc<dyn Provider> {
        Arc::new(BootstrapDouble::std_provider(
            config, model, completer, observer,
        ))
    }
}

fn model() -> ModelInfo {
    ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        8192,
    )
}

/// @covers: ProviderBootstrap::bootstrap_name — default impl reports "provider"
#[test]
fn test_bootstrap_name_default_reports_provider_happy() {
    let name = BootstrapDouble
        .bootstrap_name(ProviderBootstrapNameRequest)
        .expect("bootstrap_name ok")
        .name;
    assert_eq!(name, "provider");
}

/// @covers: ProviderBootstrap::provider — constructs a provider that reports the given model
#[test]
fn test_provider_constructs_from_config_and_model_error() {
    let provider = BootstrapDouble::provider(
        ProviderConfig::new("claude".to_string(), 0.7, 8192),
        Box::new(model()),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    );
    let name = provider.name(ProviderNameRequest).expect("name ok").name;
    assert_eq!(name, "claude");
}

/// @covers: ProviderBootstrap::std_provider — concrete constructor mirrors provider() boundary case
#[test]
fn test_std_provider_empty_model_name_edge() {
    let empty_model = ModelInfo::new(String::new(), String::new(), ModelFamily::Other, 0);
    let provider = BootstrapDouble::std_provider(
        ProviderConfig::new(String::new(), 0.0, 0),
        Box::new(empty_model),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    );
    assert!(provider
        .health_check(edge_llm_provider::HealthCheckRequest)
        .is_err());
}
