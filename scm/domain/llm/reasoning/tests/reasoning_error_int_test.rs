//! Tests for the `ReasoningError` error type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::ReasoningError;

/// @covers: ReasoningError::is_recoverable — timeout is recoverable
#[test]
fn test_is_recoverable_timeout() {
    let err = ReasoningError::Timeout { timeout_secs: 30 };
    assert!(err.is_recoverable());
}

/// @covers: ReasoningError::is_recoverable — invalid state is not recoverable
#[test]
fn test_is_recoverable_invalid_state_false() {
    let err = ReasoningError::InvalidState("bad".to_string());
    assert!(!err.is_recoverable());
}

/// @covers: ReasoningError::message — describes the failure
#[test]
fn test_message_mentions_depth() {
    let err = ReasoningError::MaxDepthExceeded { max_depth: 12 };
    assert!(err.message().contains("12"));
}

/// @covers: ReasoningError — serde round-trip
#[test]
fn test_reasoning_error_serde_roundtrip() {
    let err = ReasoningError::BudgetExhausted {
        tokens_used: 100,
        token_limit: 50,
    };
    let json = serde_json::to_string(&err).expect("serialize");
    let back: ReasoningError = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(back, ReasoningError::BudgetExhausted { .. }));
}
