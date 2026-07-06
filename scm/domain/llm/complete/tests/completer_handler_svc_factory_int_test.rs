//! SAF service tests — completer-handler factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::COMPLETER_HANDLER_SVC_FACTORY;

/// @covers: COMPLETER_HANDLER_SVC_FACTORY — equals the canonical string
#[test]
fn test_completer_handler_svc_factory_equals_canonical_happy() {
    assert_eq!(
        COMPLETER_HANDLER_SVC_FACTORY,
        "completer_handler_svc_factory"
    );
}

/// @covers: COMPLETER_HANDLER_SVC_FACTORY — is non-empty
#[test]
fn test_completer_handler_svc_factory_is_non_empty_error() {
    assert!(!COMPLETER_HANDLER_SVC_FACTORY.is_empty());
}

/// @covers: COMPLETER_HANDLER_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_completer_handler_svc_factory_is_valid_identifier_edge() {
    assert!(COMPLETER_HANDLER_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
