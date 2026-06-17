//! ADR-037 connection tests — `ProviderEndpoint` as both `Handler` and `Service`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceRegistry};
use edge_llm_provider::{
    ExecutionConfig, ExecutionMode, ExecutionStepResult, ProviderEndpoint, ProviderFactory,
    StdProviderFactory,
};
use futures::executor::block_on;

fn endpoint() -> ProviderEndpoint {
    StdProviderFactory::endpoint(ExecutionConfig::new(
        4096,
        30_000,
        true,
        false,
        ExecutionMode::Async,
    ))
}

/// @covers: ProviderEndpoint (Service face) — Service → Dispatch → Handler → core
#[test]
fn test_service_execute_delegates_through_handler_happy() {
    let ep = endpoint();
    let out = block_on(Service::execute(&ep, "ship it".to_string())).expect("service ok");
    assert!(out.reasoning.contains("ship it"));
}

/// @covers: ProviderEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_runs_core_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
    };
    let out = block_on(Handler::execute(&ep, "ship it".to_string(), ctx)).expect("handler ok");
    assert!(out.reasoning.contains("ship it"));
}

/// @covers: ProviderEndpoint — dispatch id and service name are distinct identifiers
#[test]
fn test_endpoint_handler_id_and_service_name_distinct() {
    let ep = endpoint();
    assert_eq!(Handler::id(&ep), "provider.execute_step");
    assert_eq!(Service::name(&ep), "provider");
}

/// @covers: ProviderEndpoint — consumer resolves it from a ServiceRegistry by name
#[test]
fn test_endpoint_resolves_from_service_registry() {
    let registry: ServiceRegistry<String, ExecutionStepResult> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));

    let svc = registry
        .get("provider")
        .expect("service registered under its name");
    let out = block_on(svc.execute("ship it".to_string())).expect("resolved service ok");
    assert!(out.reasoning.contains("ship it"));
}

/// @covers: ProviderEndpoint — unregistered name resolves to nothing
#[test]
fn test_endpoint_unknown_name_returns_none_edge() {
    let registry: ServiceRegistry<String, ExecutionStepResult> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));
    assert!(registry.get("nope").is_none());
}
