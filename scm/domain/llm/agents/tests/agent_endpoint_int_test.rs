//! ADR-037 connection tests — `AgentEndpoint` as both `Handler` and `Service`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceRegistry};
use edge_llm_agent::{AgentEndpoint, AgentManager, NoopAgentManager};
use futures::executor::block_on;

fn endpoint() -> AgentEndpoint {
    AgentEndpoint::new("code_review")
}

/// @covers: AgentEndpoint (Service face) — Service → Dispatch → Handler → core
#[test]
fn test_service_execute_delegates_through_handler_happy() {
    let ep = endpoint();
    let out = block_on(Service::execute(&ep, "diff".to_string())).expect("service ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: AgentEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_runs_core_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
    };
    let out = block_on(Handler::execute(&ep, "diff".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: AgentEndpoint — dispatch id and service name are distinct identifiers
#[test]
fn test_endpoint_handler_id_and_service_name_distinct() {
    let ep = endpoint();
    assert_eq!(Handler::id(&ep), "agent.execute_skill");
    assert_eq!(Service::name(&ep), "agent");
}

/// @covers: AgentEndpoint — consumer resolves it from a ServiceRegistry by name
#[test]
fn test_endpoint_resolves_from_service_registry_happy() {
    let registry: ServiceRegistry<String, String> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));

    let svc = registry
        .get("agent")
        .expect("service registered under its name");
    let out = block_on(svc.execute("diff".to_string())).expect("resolved service ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: AgentEndpoint — Service surfaces a Handler failure as ServiceError
#[test]
fn test_service_execute_empty_input_returns_error() {
    let ep = endpoint();
    let err =
        block_on(Service::execute(&ep, String::new())).expect_err("empty skill input is rejected");
    assert!(matches!(
        err,
        edge_domain_service::ServiceError::Internal(_)
    ));
}

/// @covers: AgentEndpoint — unregistered name resolves to nothing
#[test]
fn test_endpoint_unknown_name_returns_none_edge() {
    let registry: ServiceRegistry<String, String> = ServiceRegistry::new();
    registry.register(Arc::new(endpoint()));
    assert!(registry.get("nope").is_none());
}

/// @covers: AgentManager::endpoint — the manager constructs an endpoint for a skill
#[test]
fn test_endpoint_manager_targets_named_skill_happy() {
    let manager = NoopAgentManager;
    let ep = manager.endpoint("planning");
    assert_eq!(ep.skill(), "planning");
    assert_eq!(Handler::id(&ep), "agent.execute_skill");
}

/// @covers: AgentManager::endpoint — endpoint built via the manager still rejects empty input
#[test]
fn test_endpoint_manager_empty_input_returns_error() {
    let manager = NoopAgentManager;
    let ep = manager.endpoint("planning");
    let err = block_on(Service::execute(&ep, String::new())).expect_err("empty input rejected");
    assert!(matches!(
        err,
        edge_domain_service::ServiceError::Internal(_)
    ));
}

/// @covers: AgentManager::endpoint — edge: empty skill name is preserved verbatim
#[test]
fn test_endpoint_manager_empty_skill_name_edge() {
    let manager = NoopAgentManager;
    let ep = manager.endpoint("");
    assert_eq!(ep.skill(), "");
}
