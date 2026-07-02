//! End-to-end contract tests for the `TokenCounter` trait, exercised through
//! the crate's reference implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CountTokensRequest, EstimateTokensRequest, ExactnessRequest, HeuristicTokenCounter,
    TokenCounter, TokenizerNameRequest,
};

/// @covers: TokenCounter::count_tokens
#[test]
fn test_count_tokens_empty_is_zero() {
    let result = HeuristicTokenCounter::new()
        .count_tokens(CountTokensRequest { text: "" })
        .expect("count ok");
    assert_eq!(result.count, 0);
}

/// @covers: TokenCounter::estimate_tokens
#[test]
fn test_estimate_tokens_uses_char_ratio() {
    let counter = HeuristicTokenCounter::with_ratio(4);
    let result = counter
        .estimate_tokens(EstimateTokensRequest { text: "abcd" })
        .expect("estimate ok");
    assert_eq!(result.count, 1);
}

/// @covers: TokenCounter::tokenizer_name
#[test]
fn test_tokenizer_name_matches_constant() {
    let result = HeuristicTokenCounter::new()
        .tokenizer_name(TokenizerNameRequest)
        .expect("tokenizer_name ok");
    assert_eq!(result.name, HeuristicTokenCounter::TOKENIZER_NAME);
}

/// @covers: TokenCounter::is_exact
#[test]
fn test_is_exact_reports_false() {
    let result = HeuristicTokenCounter::new()
        .is_exact(ExactnessRequest)
        .expect("is_exact ok");
    assert!(!result.exact);
}
