//! Tests for `TokenUsageBuilder` setter methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::TokenUsageBuilder;

/// @covers: TokenUsageBuilder::prompt_tokens — sets prompt token count
#[test]
fn test_prompt_tokens() {
    let u = TokenUsageBuilder::new().prompt_tokens(100).build();
    assert_eq!(u.prompt_tokens, 100);
}

/// @covers: TokenUsageBuilder::prompt_tokens — zero is the minimum
#[test]
fn test_prompt_tokens_zero_minimum_edge() {
    let u = TokenUsageBuilder::new().prompt_tokens(0).build();
    assert_eq!(u.prompt_tokens, 0);
}

/// @covers: TokenUsageBuilder::completion_tokens — sets completion token count
#[test]
fn test_completion_tokens() {
    let u = TokenUsageBuilder::new().completion_tokens(50).build();
    assert_eq!(u.completion_tokens, 50);
}

/// @covers: TokenUsageBuilder::cache_read_input_tokens — sets cache read tokens
#[test]
fn test_cache_read_input_tokens() {
    let u = TokenUsageBuilder::new().cache_read_input_tokens(20).build();
    assert_eq!(u.cache_read_input_tokens, 20);
}

/// @covers: TokenUsageBuilder::cache_creation_input_tokens — sets cache creation tokens
#[test]
fn test_cache_creation_input_tokens() {
    let u = TokenUsageBuilder::new().cache_creation_input_tokens(10).build();
    assert_eq!(u.cache_creation_input_tokens, 10);
}

/// @covers: TokenUsageBuilder::build — total tokens sums prompt and completion
#[test]
fn test_build() {
    let u = TokenUsageBuilder::new().prompt_tokens(100).completion_tokens(50).build();
    assert_eq!(u.total_tokens, 150);
}

/// @covers: TokenUsageBuilder::build — zero when no tokens set
#[test]
fn test_build_zero_total_edge() {
    let u = TokenUsageBuilder::new().build();
    assert_eq!(u.total_tokens, 0);
}
