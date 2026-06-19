#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_ENDPOINT_SVC constant and agent_handler factory.

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_agent::{AgentEndpoint, NoopAgentManager, AGENT_ENDPOINT_SVC};
use futures::executor::block_on;

/// @covers: AGENT_ENDPOINT_SVC constant
#[test]
fn test_svc_agent_endpoint_svc_happy_constant_equals_agent_endpoint() {
    assert_eq!(AGENT_ENDPOINT_SVC, "agent_endpoint");
}

/// @covers: AGENT_ENDPOINT_SVC constant
#[test]
fn test_svc_agent_endpoint_svc_error_constant_not_empty() {
    assert!(!AGENT_ENDPOINT_SVC.is_empty());
}

/// @covers: AGENT_ENDPOINT_SVC constant
#[test]
fn test_svc_agent_endpoint_svc_edge_constant_is_valid_identifier() {
    assert!(AGENT_ENDPOINT_SVC
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}

/// @covers: agent_handler factory — constructed and consumed via the Handler face
#[test]
fn test_svc_agent_handler_happy_reexport_executes_as_handler() {
    let h = NoopAgentManager::agent_handler("summarize");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "doc".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "summarize:doc");
}

/// @covers: agent_handler factory — empty input is rejected through the Handler face
#[test]
fn test_svc_agent_handler_error_reexport_rejects_empty_input() {
    let h = NoopAgentManager::agent_handler("summarize");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    assert!(block_on(Handler::execute(&h, String::new(), ctx)).is_err());
}

/// @covers: NoopAgentManager::agent_handler — routes input to named skill
#[test]
fn test_agent_handler_with_valid_skill_happy() {
    let h = NoopAgentManager::agent_handler("summarize");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "content".to_string(), ctx)).expect("ok");
    assert_eq!(out, "summarize:content");
}

/// @covers: NoopAgentManager::agent_handler — empty input is rejected
#[test]
fn test_agent_handler_on_empty_input_error() {
    let h = NoopAgentManager::agent_handler("summarize");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    assert!(block_on(Handler::execute(&h, String::new(), ctx)).is_err());
}

/// @covers: NoopAgentManager::agent_handler — empty skill name is preserved as the skill label
#[test]
fn test_agent_handler_empty_skill_name_edge() {
    let h = NoopAgentManager::agent_handler("");
    assert_eq!(Handler::id(&h), "agent.execute_skill");
}
