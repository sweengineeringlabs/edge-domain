//! SAF service tests — processor factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::PROCESSOR_SVC_FACTORY;

/// @covers: PROCESSOR_SVC_FACTORY — equals the canonical string
#[test]
fn test_processor_svc_factory_equals_canonical_happy() {
    assert_eq!(PROCESSOR_SVC_FACTORY, "processor_svc_factory");
}

/// @covers: PROCESSOR_SVC_FACTORY — is non-empty
#[test]
fn test_processor_svc_factory_is_non_empty_error() {
    assert!(!PROCESSOR_SVC_FACTORY.is_empty());
}

/// @covers: PROCESSOR_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_processor_svc_factory_is_valid_identifier_edge() {
    assert!(PROCESSOR_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
