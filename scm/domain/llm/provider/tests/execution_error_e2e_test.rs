//! Tests for `ExecutionError` — retryability and messaging.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ExecutionError;
use std::time::Duration;

/// @covers: ExecutionError::is_retryable — rate-limited errors are retryable
#[test]
fn test_is_retryable_rate_limited_happy() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(1000),
    };
    assert!(err.is_retryable());
}

/// @covers: ExecutionError::is_retryable — auth failures are not retryable
#[test]
fn test_is_retryable_auth_failed_error() {
    let err = ExecutionError::AuthenticationFailed("bad key".to_string());
    assert!(!err.is_retryable());
}

/// @covers: ExecutionError::retry_after — returns duration for rate-limited errors
#[test]
fn test_retry_after_returns_duration_happy() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(2000),
    };
    assert_eq!(err.retry_after(), Some(Duration::from_millis(2000)));
}

/// @covers: ExecutionError::retry_after — returns None for non-retryable errors
#[test]
fn test_retry_after_none_for_auth_error() {
    let err = ExecutionError::AuthenticationFailed("x".to_string());
    assert_eq!(err.retry_after(), None);
}

/// @covers: ExecutionError::message — context window message contains descriptor
#[test]
fn test_message_mentions_context_window_happy() {
    let err = ExecutionError::ContextWindowExceeded {
        max_tokens: 8192,
        requested: 9000,
    };
    assert!(err.message().contains("Context window"));
}

/// @covers: ExecutionError — serializes and deserializes correctly
#[test]
fn test_execution_error_serde_roundtrip_edge() {
    let err = ExecutionError::Timeout {
        duration_ms: 30_000,
    };
    let json = serde_json::to_string(&err).expect("serialize");
    let back: ExecutionError = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(back, ExecutionError::Timeout { .. }));
}

/// @covers: ExecutionError::is_retryable — network errors are retryable
#[test]
fn test_is_retryable_network_error_happy() {
    let err = ExecutionError::NetworkError("connection refused".to_string());
    assert!(err.is_retryable());
}

/// @covers: ExecutionError::message — network error message contains reason
#[test]
fn test_network_error_message_contains_reason() {
    let err = ExecutionError::NetworkError("DNS lookup failed".to_string());
    assert!(err.message().contains("DNS lookup failed"));
}

/// @covers: ExecutionError::NetworkError — serializes with correct serde rename
#[test]
fn test_network_error_serde_roundtrip_edge() {
    let err = ExecutionError::NetworkError("TLS handshake timeout".to_string());
    let json = serde_json::to_string(&err).expect("serialize");
    assert!(json.contains("network_error"));
    let back: ExecutionError = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(back, ExecutionError::NetworkError(_)));
}
