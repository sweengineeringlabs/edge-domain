//! Tests for the `ExecutionError` error type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ExecutionError;
use std::time::Duration;

/// @covers: ExecutionError::is_retryable — rate limit is retryable
#[test]
fn test_is_retryable_rate_limited() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(1000),
    };
    assert!(err.is_retryable());
}

/// @covers: ExecutionError::is_retryable — auth failure is not retryable
#[test]
fn test_is_retryable_auth_failed_false() {
    let err = ExecutionError::AuthenticationFailed("bad key".to_string());
    assert!(!err.is_retryable());
}

/// @covers: ExecutionError::retry_after — returns the configured duration
#[test]
fn test_retry_after_returns_duration() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(2000),
    };
    assert_eq!(err.retry_after(), Some(Duration::from_millis(2000)));
}

/// @covers: ExecutionError::retry_after — none for non-temporal errors
#[test]
fn test_retry_after_none_for_auth() {
    let err = ExecutionError::AuthenticationFailed("x".to_string());
    assert_eq!(err.retry_after(), None);
}

/// @covers: ExecutionError::message — describes the failure
#[test]
fn test_message_mentions_context_window() {
    let err = ExecutionError::ContextWindowExceeded {
        max_tokens: 8192,
        requested: 9000,
    };
    assert!(err.message().contains("Context window"));
}

/// @covers: ExecutionError — serde round-trip
#[test]
fn test_execution_error_serde_roundtrip() {
    let err = ExecutionError::Timeout {
        duration_ms: 30_000,
    };
    let json = serde_json::to_string(&err).expect("serialize");
    let back: ExecutionError = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(back, ExecutionError::Timeout { .. }));
}
