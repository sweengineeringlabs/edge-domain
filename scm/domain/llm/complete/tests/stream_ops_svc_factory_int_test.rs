//! SAF service tests — stream-ops factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::STREAM_OPS_SVC_FACTORY;

/// @covers: STREAM_OPS_SVC_FACTORY — equals the canonical string
#[test]
fn test_stream_ops_svc_factory_equals_canonical_happy() {
    assert_eq!(STREAM_OPS_SVC_FACTORY, "stream_ops_svc_factory");
}

/// @covers: STREAM_OPS_SVC_FACTORY — is non-empty
#[test]
fn test_stream_ops_svc_factory_is_non_empty_error() {
    assert!(!STREAM_OPS_SVC_FACTORY.is_empty());
}

/// @covers: STREAM_OPS_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_stream_ops_svc_factory_is_valid_identifier_edge() {
    assert!(STREAM_OPS_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
