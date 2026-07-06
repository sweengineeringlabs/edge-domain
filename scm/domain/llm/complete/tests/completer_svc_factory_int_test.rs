//! SAF service tests — completer factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::COMPLETER_SVC_FACTORY;

/// @covers: COMPLETER_SVC_FACTORY — equals the canonical string
#[test]
fn test_completer_svc_factory_equals_canonical_happy() {
    assert_eq!(COMPLETER_SVC_FACTORY, "completer_svc_factory");
}

/// @covers: COMPLETER_SVC_FACTORY — is non-empty
#[test]
fn test_completer_svc_factory_is_non_empty_error() {
    assert!(!COMPLETER_SVC_FACTORY.is_empty());
}

/// @covers: COMPLETER_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_completer_svc_factory_is_valid_identifier_edge() {
    assert!(COMPLETER_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
