//! Tests for VALIDATOR_SVC constant and Validator trait re-export.

/// @covers VALIDATOR_SVC constant
#[test]
fn test_svc_validator_svc_happy_constant_equals_validator() {
    assert_eq!(edge_domain_agent::VALIDATOR_SVC, "validator");
}

/// @covers VALIDATOR_SVC constant
#[test]
fn test_svc_validator_svc_error_constant_not_empty() {
    assert!(!edge_domain_agent::VALIDATOR_SVC.is_empty());
}

/// @covers VALIDATOR_SVC constant
#[test]
fn test_svc_validator_svc_edge_constant_is_valid_identifier() {
    let svc = edge_domain_agent::VALIDATOR_SVC;
    assert!(svc.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
}
