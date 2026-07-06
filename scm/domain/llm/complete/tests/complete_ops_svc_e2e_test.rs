//! Scenario coverage for the `complete_ops_svc` SAF surface.

use edge_llm_complete::{
    CompleteOps, CompletionCheckRequest, Message, NoopCompleter, COMPLETE_OPS_SVC,
};

#[test]
fn test_complete_ops_svc_constant_is_expected_value_happy() {
    assert_eq!(COMPLETE_OPS_SVC, "complete_ops");
}

#[test]
fn test_complete_ops_svc_constant_is_nonempty_error() {
    assert!(!COMPLETE_OPS_SVC.is_empty());
}

#[test]
fn test_complete_ops_check_valid_request_returns_ok_edge() {
    let req = NoopCompleter::assemble("model".to_string(), vec![Message::user("hi")]);
    assert!(matches!(
        NoopCompleter.check(CompletionCheckRequest { request: &req }),
        Ok(())
    ));
}
