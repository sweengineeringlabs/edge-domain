//! Tests for `TokenUsage`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::TokenUsage;

/// @covers: TokenUsage::new — sums prompt and completion tokens
#[test]
fn test_new_sums_prompt_and_completion_happy() {
    let usage = TokenUsage::new(100, 50, 0, 0);
    assert_eq!(usage.total_tokens, 150);
}

/// @covers: TokenUsage::cache_hit — true when cache reads exist
#[test]
fn test_cache_hit_true_with_reads_happy() {
    let usage = TokenUsage::new(100, 50, 20, 0);
    assert!(usage.cache_hit());
}

/// @covers: TokenUsage::cache_hit — false without cache reads
#[test]
fn test_cache_hit_false_without_reads_error() {
    let usage = TokenUsage::new(100, 50, 0, 10);
    assert!(!usage.cache_hit());
}

/// @covers: TokenUsage::total_with_cache — includes all token operations
#[test]
fn test_total_with_cache_includes_cache_ops_happy() {
    let usage = TokenUsage::new(100, 50, 20, 10);
    assert_eq!(usage.total_with_cache(), 180);
}

/// @covers: TokenUsage — serializes and deserializes correctly
#[test]
fn test_token_usage_serde_roundtrip_edge() {
    let usage = TokenUsage::new(1, 2, 3, 4);
    let json = serde_json::to_string(&usage).expect("serialize");
    let back: TokenUsage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(usage, back);
}
