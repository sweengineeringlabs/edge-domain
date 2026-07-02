//! SAF service tests — reasoning factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::REASONING_SVC_FACTORY;

/// @covers: REASONING_SVC_FACTORY — equals the canonical string
#[test]
fn test_reasoning_svc_factory_equals_canonical_happy() {
    assert_eq!(REASONING_SVC_FACTORY, "reasoning_svc_factory");
}

/// @covers: REASONING_SVC_FACTORY — is non-empty
#[test]
fn test_reasoning_svc_factory_is_non_empty_error() {
    assert!(!REASONING_SVC_FACTORY.is_empty());
}

/// @covers: REASONING_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_reasoning_svc_factory_is_valid_identifier_edge() {
    assert!(REASONING_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
