#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_ENDPOINT_SVC constant and DefaultAgent re-export.

use edge_domain_service::Service;
use edge_llm_agent::{DefaultAgent, AGENT_ENDPOINT_SVC};
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

/// @covers: DefaultAgent re-export — constructed and consumed via the Service face
#[test]
fn test_svc_agent_endpoint_happy_reexport_executes_as_service() {
    let ep = DefaultAgent::new("summarize");
    let out = block_on(Service::execute(&ep, "doc".to_string())).expect("service ok");
    assert_eq!(out, "summarize:doc");
}

/// @covers: DefaultAgent re-export — empty input is rejected through the Service face
#[test]
fn test_svc_agent_endpoint_error_reexport_rejects_empty_input() {
    let ep = DefaultAgent::new("summarize");
    let err = block_on(Service::execute(&ep, String::new())).expect_err("empty input rejected");
    assert!(matches!(
        err,
        edge_domain_service::ServiceError::Internal(_)
    ));
}
