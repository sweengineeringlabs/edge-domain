#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_ENDPOINT_SVC constant and AgentEndpoint re-export.

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_agent::{AgentEndpoint, AGENT_ENDPOINT_SVC};
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

/// @covers: AgentEndpoint re-export — constructed and consumed via the Handler face
#[test]
fn test_svc_agent_endpoint_happy_reexport_executes_as_handler() {
    let ep = AgentEndpoint::new("summarize");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    let out = block_on(Handler::execute(&ep, "doc".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "summarize:doc");
}

/// @covers: AgentEndpoint re-export — empty input is rejected through the Handler face
#[test]
fn test_svc_agent_endpoint_error_reexport_rejects_empty_input() {
    let ep = AgentEndpoint::new("summarize");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    assert!(block_on(Handler::execute(&ep, String::new(), ctx)).is_err());
}
