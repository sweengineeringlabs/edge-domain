//! Handler integration tests — `AgentEndpoint` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_agent::{AgentEndpoint, AgentManager, NoopAgentManager};
use futures::executor::block_on;

fn endpoint() -> AgentEndpoint {
    AgentEndpoint::new("code_review")
}

/// @covers: AgentEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_returns_skill_colon_input_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    let out = block_on(Handler::execute(&ep, "diff".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: AgentEndpoint — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    assert_eq!(Handler::id(&endpoint()), "agent.execute_skill");
}

/// @covers: AgentEndpoint — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    assert_eq!(Handler::pattern(&endpoint()), "agent/execute_skill");
}

/// @covers: AgentEndpoint — empty input surfaces a handler error
#[test]
fn test_handler_execute_empty_input_returns_error() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    assert!(block_on(Handler::execute(&ep, String::new(), ctx)).is_err());
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
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    assert!(block_on(Handler::execute(&ep, String::new(), ctx)).is_err());
}

/// @covers: AgentManager::endpoint — edge: empty skill name is preserved verbatim
#[test]
fn test_endpoint_manager_empty_skill_name_edge() {
    let manager = NoopAgentManager;
    let ep = manager.endpoint("");
    assert_eq!(ep.skill(), "");
}
