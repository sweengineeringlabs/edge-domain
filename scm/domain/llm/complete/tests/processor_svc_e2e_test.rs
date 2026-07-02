//! Scenario coverage for the `processor_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{EchoCompleter, Message, ProcessingRequest, Processor, PROCESSOR_SVC};
use futures::executor::block_on;

#[test]
fn test_processor_svc_constant_is_expected_value_happy() {
    assert_eq!(PROCESSOR_SVC, "processor");
}

#[test]
fn test_processor_svc_constant_is_nonempty_error() {
    assert!(!PROCESSOR_SVC.is_empty());
}

#[test]
fn test_processor_trait_accessible_via_svc_surface_edge() {
    use edge_llm_complete::CompletionRequest;
    let req = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let resp = block_on(EchoCompleter.process(ProcessingRequest { request: &req })).unwrap();
    assert_eq!(resp.content, Some("hi".to_string()));
}
