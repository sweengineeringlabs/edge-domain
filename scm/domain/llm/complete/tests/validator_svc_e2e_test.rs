//! Scenario coverage for the `validator_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    CompleteError, CompletionRequest, EchoCompleter, ValidationRequest, Validator, VALIDATOR_SVC,
};

#[test]
fn test_validator_svc_constant_is_expected_value_happy() {
    assert_eq!(VALIDATOR_SVC, "validator");
}

#[test]
fn test_validator_svc_constant_is_nonempty_error() {
    assert!(!VALIDATOR_SVC.is_empty());
}

#[test]
fn test_validator_rejects_empty_model_edge() {
    let req = CompletionRequest::new("", vec![]);
    let err = EchoCompleter
        .validate(ValidationRequest { request: &req })
        .unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}
