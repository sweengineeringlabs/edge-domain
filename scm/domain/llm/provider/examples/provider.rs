//! Basic `edge-llm-provider` usage example.
#![allow(clippy::expect_used)]

use std::sync::Arc;

use edge_domain_observer::StdObserveFactory;
use edge_llm_complete::NoopCompleter;
use edge_llm_provider::{
    ExecutionMode, HealthCheckRequest, ModelFamily, ModelFamilyRequest, ModelInfo, Provider,
    ProviderBootstrap, ProviderConfig, ProviderNameRequest, StdProviderFactory,
};

fn main() {
    let config = ProviderConfig::new("claude".to_string(), 0.7, 200_000);
    let info = ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        200_000,
    );

    let provider = StdProviderFactory::provider(
        config,
        Box::new(info),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    );
    println!(
        "provider: {}",
        provider.name(ProviderNameRequest).expect("name").name
    );
    println!(
        "family: {:?}",
        provider
            .model_family(ModelFamilyRequest)
            .expect("family")
            .family
    );
    println!(
        "healthy: {:?}",
        provider.health_check(HealthCheckRequest).is_ok()
    );

    let mode = ExecutionMode::Streaming;
    println!("streaming mode: {}", mode.is_streaming());
}
