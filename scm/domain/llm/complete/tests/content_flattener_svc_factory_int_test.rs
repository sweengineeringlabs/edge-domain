//! SAF service tests — content-flattener factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::CONTENT_FLATTENER_SVC_FACTORY;

/// @covers: CONTENT_FLATTENER_SVC_FACTORY — equals the canonical string
#[test]
fn test_content_flattener_svc_factory_equals_canonical_happy() {
    assert_eq!(
        CONTENT_FLATTENER_SVC_FACTORY,
        "content_flattener_svc_factory"
    );
}

/// @covers: CONTENT_FLATTENER_SVC_FACTORY — is non-empty
#[test]
fn test_content_flattener_svc_factory_is_non_empty_error() {
    assert!(!CONTENT_FLATTENER_SVC_FACTORY.is_empty());
}

/// @covers: CONTENT_FLATTENER_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_content_flattener_svc_factory_is_valid_identifier_edge() {
    assert!(CONTENT_FLATTENER_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
