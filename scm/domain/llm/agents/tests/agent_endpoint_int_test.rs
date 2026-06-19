//! Handler integration tests — `agent_handler` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_agent::{AgentEndpoint, NoopAgentManager};
use futures::executor::block_on;

/// @covers: agent_handler (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_returns_skill_colon_input_happy() {
    let h = NoopAgentManager::agent_handler("code_review");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "diff".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: agent_handler — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    let h = NoopAgentManager::agent_handler("any_skill");
    assert_eq!(Handler::id(&h), "agent.execute_skill");
}

/// @covers: agent_handler — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    let h = NoopAgentManager::agent_handler("any_skill");
    assert_eq!(Handler::pattern(&h), "agent/execute_skill");
}

/// @covers: agent_handler — empty input surfaces a handler error
#[test]
fn test_handler_execute_empty_input_returns_error() {
    let h = NoopAgentManager::agent_handler("code_review");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    assert!(block_on(Handler::execute(&h, String::new(), ctx)).is_err());
}

/// @covers: agent_handler — targets the named skill in its output
#[test]
fn test_handler_targets_named_skill_happy() {
    let h = NoopAgentManager::agent_handler("planning");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "a task".to_string(), ctx)).expect("ok");
    assert_eq!(out, "planning:a task");
}

/// @covers: agent_handler — edge: empty skill name is preserved verbatim
#[test]
fn test_handler_empty_skill_name_edge() {
    let h = NoopAgentManager::agent_handler("");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "input".to_string(), ctx)).expect("ok");
    assert_eq!(out, ":input");
}
