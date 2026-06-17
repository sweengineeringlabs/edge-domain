//! ADR-037 connection tests — `PromptEndpoint` as both `Handler` and `Service`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceRegistry};
use edge_llm_prompt::{
    PromptEndpoint, PromptFactory, PromptMetadata, RenderContext, StdPromptFactory, Variable,
    VariableType,
};
use futures::executor::block_on;

fn endpoint() -> PromptEndpoint {
    let var = Variable::new("name".to_string(), VariableType::String);
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vec![var],
    );
    StdPromptFactory::endpoint("Hello {{name}}".to_string(), metadata)
}

fn context() -> RenderContext {
    RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"))
}

/// @covers: PromptEndpoint (Service face) — Service → Dispatch → Handler → core
#[test]
fn test_service_execute_delegates_through_handler_happy() {
    let ep = endpoint();
    let out = block_on(Service::execute(&ep, context())).expect("service ok");
    assert_eq!(out, "Hello Ada");
}

/// @covers: PromptEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_runs_core_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
    };
    let out = block_on(Handler::execute(&ep, context(), ctx)).expect("handler ok");
    assert_eq!(out, "Hello Ada");
}

/// @covers: PromptEndpoint — dispatch id and service name are distinct identifiers
#[test]
fn test_endpoint_handler_id_and_service_name_distinct() {
    let ep = endpoint();
    assert_eq!(Handler::id(&ep), "prompt.render");
    assert_eq!(Service::name(&ep), "prompt");
}

/// @covers: PromptEndpoint — a missing required variable surfaces an error through the pipeline
#[test]
fn test_service_execute_missing_variable_errors_error() {
    let ep = endpoint();
    assert!(block_on(Service::execute(&ep, RenderContext::new())).is_err());
}

/// @covers: PromptEndpoint — consumer resolves it from a ServiceRegistry by name
#[test]
fn test_endpoint_resolves_from_service_registry() {
    let registry: ServiceRegistry<RenderContext, String> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));

    let svc = registry
        .get("prompt")
        .expect("service registered under its name");
    let out = block_on(svc.execute(context())).expect("resolved service ok");
    assert_eq!(out, "Hello Ada");
}

/// @covers: PromptEndpoint — unregistered name resolves to nothing
#[test]
fn test_endpoint_unknown_name_returns_none_edge() {
    let registry: ServiceRegistry<RenderContext, String> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));
    assert!(registry.get("nope").is_none());
}
