//! Tests for the `PromptError` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::PromptError;

/// @covers: PromptError::is_recoverable — cache errors are recoverable
#[test]
fn test_prompt_error_cache_is_recoverable() {
    assert!(PromptError::CacheError("x".to_string()).is_recoverable());
}

/// @covers: PromptError::is_recoverable — missing variable is not recoverable
#[test]
fn test_prompt_error_missing_variable_not_recoverable() {
    let err = PromptError::MissingVariable {
        variable_name: "a".to_string(),
    };
    assert!(!err.is_recoverable());
}

/// @covers: PromptError::message — incomplete context lists missing names
#[test]
fn test_prompt_error_message_lists_missing() {
    let err = PromptError::IncompleteContext {
        missing_variables: vec!["a".to_string(), "b".to_string()],
    };
    assert!(err.message().contains("a, b"));
}

/// @covers: PromptError — serde round-trip preserves the variant
#[test]
fn test_prompt_error_serde_roundtrip() {
    let err = PromptError::RenderFailed("boom".to_string());
    let json = serde_json::to_string(&err).expect("serialize");
    let back: PromptError = serde_json::from_str(&json).expect("deserialize");
    assert!(back.message().contains("boom"));
}
