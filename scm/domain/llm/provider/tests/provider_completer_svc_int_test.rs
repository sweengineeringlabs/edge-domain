//! Integration tests for the `provider_completer_svc` SAF contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::PROVIDER_COMPLETER_SVC;

/// @covers: PROVIDER_COMPLETER_SVC — constant holds the canonical value
#[test]
fn test_provider_completer_svc_equals_canonical_happy() {
    assert_eq!(PROVIDER_COMPLETER_SVC, "provider_completer");
}

/// @covers: PROVIDER_COMPLETER_SVC — value is non-empty
#[test]
fn test_provider_completer_svc_is_non_empty_error() {
    assert!(!PROVIDER_COMPLETER_SVC.is_empty());
}

/// @covers: PROVIDER_COMPLETER_SVC — value is a valid identifier (no spaces, no dots)
#[test]
fn test_provider_completer_svc_is_valid_identifier_edge() {
    assert!(!PROVIDER_COMPLETER_SVC.contains(' '));
    assert!(!PROVIDER_COMPLETER_SVC.contains('.'));
}
