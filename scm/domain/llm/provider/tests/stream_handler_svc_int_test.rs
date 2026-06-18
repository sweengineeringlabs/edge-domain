//! SAF service tests — stream-handler contract identifier.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::STREAM_HANDLER_SVC;

/// @covers: STREAM_HANDLER_SVC — equals the canonical string
#[test]
fn test_stream_handler_svc_equals_canonical_happy() {
    assert_eq!(STREAM_HANDLER_SVC, "stream_handler");
}

/// @covers: STREAM_HANDLER_SVC — is non-empty
#[test]
fn test_stream_handler_svc_is_non_empty_error() {
    assert!(!STREAM_HANDLER_SVC.is_empty());
}

/// @covers: STREAM_HANDLER_SVC — contains only lowercase ASCII and underscores
#[test]
fn test_stream_handler_svc_is_valid_identifier_edge() {
    assert!(STREAM_HANDLER_SVC
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
