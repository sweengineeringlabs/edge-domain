//! Tests for the `StdProviderFactory` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_observe::StdObserveFactory;
use edge_llm_complete::NoopCompleter;
use edge_llm_provider::{Provider, ProviderBootstrap, StdProviderFactory};

/// @covers: StdProviderFactory — std_factory returns the factory instance
#[test]
fn test_std_provider_bootstrap_std_factory_returns_instance() {
    let _factory: StdProviderFactory = StdProviderFactory::std_factory();
}

/// @covers: StdProviderFactory — is zero-sized
#[test]
fn test_std_provider_bootstrap_is_zero_sized() {
    assert_eq!(std::mem::size_of::<StdProviderFactory>(), 0);
}

/// @covers: StdProviderFactory — builds a provider via the factory
#[test]
fn test_std_provider_bootstrap_builds_provider() {
    use edge_llm_provider::{ModelFamily, ModelInfo, ProviderConfig};
    let config = ProviderConfig::new("claude".to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    assert_eq!(
        StdProviderFactory::provider(
            config,
            info,
            Arc::new(NoopCompleter),
            StdObserveFactory::noop_arc_observe_context()
        )
        .name(),
        "claude"
    );
}
