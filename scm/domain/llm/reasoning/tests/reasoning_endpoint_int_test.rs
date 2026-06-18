//! ADR-037 connection tests — `ReasoningEndpoint` as both `Handler` and `Service`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceRegistry};
use edge_llm_reasoning::{
    ReasoningEndpoint, ReasoningFactory, ReasoningPattern, StdReasoningFactory, ThinkingProcess,
};
use futures::executor::block_on;

fn endpoint() -> ReasoningEndpoint {
    StdReasoningFactory::endpoint(ReasoningPattern::ChainOfThought)
}

/// @covers: ReasoningEndpoint (Service face) — Service → Dispatch → Handler → core
#[test]
fn test_service_execute_delegates_through_handler_happy() {
    let ep = endpoint();
    let out = block_on(Service::execute(&ep, "solve x".to_string())).expect("service ok");
    assert!(out.is_complete);
    assert_eq!(out.conclusion.as_deref(), Some("conclusion for: solve x"));
}

/// @covers: ReasoningEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_runs_core_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
    };
    let out = block_on(Handler::execute(&ep, "solve x".to_string(), ctx)).expect("handler ok");
    assert!(out.is_complete);
    assert_eq!(out.step_count(), 3);
}

/// @covers: ReasoningEndpoint — dispatch id and service name are distinct identifiers
#[test]
fn test_endpoint_handler_id_and_service_name_distinct() {
    let ep = endpoint();
    assert_eq!(Handler::id(&ep), "reasoning.reason");
    assert_eq!(Service::name(&ep), "reasoning");
    assert_ne!(Handler::id(&ep), Service::name(&ep));
}

/// @covers: ReasoningEndpoint — consumer resolves it from a ServiceRegistry by name
#[test]
fn test_endpoint_resolves_from_service_registry() {
    let registry: ServiceRegistry<String, ThinkingProcess> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));

    let svc = registry
        .get("reasoning")
        .expect("service registered under its name");
    let out = block_on(svc.execute("solve x".to_string())).expect("resolved service ok");
    assert!(out.is_complete);
    let confidence = out.average_confidence();
    assert!((confidence - 0.9).abs() < 0.001);
}

/// @covers: ReasoningEndpoint — unregistered name resolves to nothing
#[test]
fn test_endpoint_unknown_name_returns_none_edge() {
    let registry: ServiceRegistry<String, ThinkingProcess> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));
    assert!(registry.get("nope").is_none());
}

/// @covers: ReasoningEndpoint (Service face) — blank problem fails through the pipeline
#[test]
fn test_service_execute_blank_problem_errors_error() {
    let ep = endpoint();
    assert!(block_on(Service::execute(&ep, "   ".to_string())).is_err());
}
