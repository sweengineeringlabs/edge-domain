//! Tests for the `TokenUsage` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::TokenUsage;

/// @covers: TokenUsage::new — totals prompt + completion
#[test]
fn test_new_sums_prompt_and_completion() {
    let usage = TokenUsage::new(100, 50, 0, 0);
    assert_eq!(usage.total_tokens, 150);
}

/// @covers: TokenUsage::cache_hit — true when cache reads occurred
#[test]
fn test_cache_hit_true_with_reads() {
    let usage = TokenUsage::new(100, 50, 20, 0);
    assert!(usage.cache_hit());
}

/// @covers: TokenUsage::cache_hit — false without cache reads
#[test]
fn test_cache_hit_false_without_reads() {
    let usage = TokenUsage::new(100, 50, 0, 10);
    assert!(!usage.cache_hit());
}

/// @covers: TokenUsage::total_with_cache — includes cache operations
#[test]
fn test_total_with_cache_includes_cache_ops() {
    let usage = TokenUsage::new(100, 50, 20, 10);
    assert_eq!(usage.total_with_cache(), 180);
}

/// @covers: TokenUsage — serde round-trip
#[test]
fn test_token_usage_serde_roundtrip() {
    let usage = TokenUsage::new(1, 2, 3, 4);
    let json = serde_json::to_string(&usage).expect("serialize");
    let back: TokenUsage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(usage, back);
}
