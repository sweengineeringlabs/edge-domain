//! SAF service tests — cacheable-message factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::CACHEABLE_MESSAGE_SVC_FACTORY;

/// @covers: CACHEABLE_MESSAGE_SVC_FACTORY — equals the canonical string
#[test]
fn test_cacheable_message_svc_factory_equals_canonical_happy() {
    assert_eq!(
        CACHEABLE_MESSAGE_SVC_FACTORY,
        "cacheable_message_svc_factory"
    );
}

/// @covers: CACHEABLE_MESSAGE_SVC_FACTORY — is non-empty
#[test]
fn test_cacheable_message_svc_factory_is_non_empty_error() {
    assert!(!CACHEABLE_MESSAGE_SVC_FACTORY.is_empty());
}

/// @covers: CACHEABLE_MESSAGE_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_cacheable_message_svc_factory_is_valid_identifier_edge() {
    assert!(CACHEABLE_MESSAGE_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
