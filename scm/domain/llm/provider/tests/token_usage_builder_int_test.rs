//! Tests for `TokenUsageBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::TokenUsageBuilder;

/// @covers: TokenUsageBuilder::build — computes the total
#[test]
fn test_token_usage_builder_computes_total() {
    let usage = TokenUsageBuilder::new()
        .prompt_tokens(100)
        .completion_tokens(50)
        .cache_read_input_tokens(20)
        .build();
    assert_eq!(usage.total_tokens, 150);
    assert!(usage.cache_hit());
}

/// @covers: TokenUsageBuilder::default — all zero
#[test]
fn test_token_usage_builder_defaults_zero() {
    let usage = TokenUsageBuilder::new().build();
    assert_eq!(usage.total_tokens, 0);
    assert!(!usage.cache_hit());
}
