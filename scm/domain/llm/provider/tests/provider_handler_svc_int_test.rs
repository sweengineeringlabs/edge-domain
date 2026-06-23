//! SAF integration tests — `provider_handler_svc` factory methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;
use edge_llm_provider::{EchoExecutionModel, ExecutionConfig, ExecutionMode, StdProviderFactory};
use futures::executor::block_on;
use std::sync::Arc;

fn config(tokens: u32) -> ExecutionConfig {
    ExecutionConfig::new(tokens, 30_000, true, false, ExecutionMode::Async)
}

// ── StdProviderFactory::provider_handler ─────────────────────────────────────

/// @covers: StdProviderFactory::provider_handler
#[test]
fn test_provider_handler_custom_model_happy_executes_step() {
    let model = Arc::new(EchoExecutionModel::new(config(4096)));
    let h = StdProviderFactory::provider_handler(model);
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let result = block_on(h.execute(
        "step input".to_string(),
        HandlerContext::new(&security, &bus, observer.as_ref()),
    ))
    .expect("ok");
    assert!(!result.reasoning.is_empty());
}

/// @covers: StdProviderFactory::provider_handler
#[test]
fn test_provider_handler_zero_token_budget_error_returns_err() {
    let model = Arc::new(EchoExecutionModel::new(config(0)));
    let h = StdProviderFactory::provider_handler(model);
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    assert!(block_on(h.execute(
        "step".to_string(),
        HandlerContext::new(&security, &bus, observer.as_ref())
    ))
    .is_err());
}

/// @covers: StdProviderFactory::provider_handler
#[test]
fn test_provider_handler_edge_dispatch_id_is_stable() {
    let model = Arc::new(EchoExecutionModel::new(config(4096)));
    let h = StdProviderFactory::provider_handler(model);
    assert_eq!(h.id(), "provider.execute_step");
}

// ── StdProviderFactory::default_provider_handler ─────────────────────────────

/// @covers: StdProviderFactory::default_provider_handler
#[test]
fn test_default_provider_handler_happy_returns_non_empty_reasoning() {
    let h = StdProviderFactory::default_provider_handler(config(4096));
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let result = block_on(h.execute(
        "echo this".to_string(),
        HandlerContext::new(&security, &bus, observer.as_ref()),
    ))
    .expect("ok");
    assert!(!result.reasoning.is_empty());
}

/// @covers: StdProviderFactory::default_provider_handler
#[test]
fn test_default_provider_handler_zero_budget_error_propagates() {
    let h = StdProviderFactory::default_provider_handler(config(0));
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    assert!(block_on(h.execute(
        "echo".to_string(),
        HandlerContext::new(&security, &bus, observer.as_ref())
    ))
    .is_err());
}

/// @covers: StdProviderFactory::default_provider_handler
#[test]
fn test_default_provider_handler_edge_id_matches_provider_handler() {
    let default_h = StdProviderFactory::default_provider_handler(config(4096));
    let custom_h =
        StdProviderFactory::provider_handler(Arc::new(EchoExecutionModel::new(config(4096))));
    assert_eq!(default_h.id(), custom_h.id());
}
