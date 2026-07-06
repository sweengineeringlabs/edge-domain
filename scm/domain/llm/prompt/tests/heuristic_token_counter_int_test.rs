//! Tests for the `HeuristicTokenCounter` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CountTokensRequest, EstimateTokensRequest, ExactnessRequest, HeuristicTokenCounter,
    TokenCounter,
};

/// @covers: HeuristicTokenCounter::new — counts tokens in text
#[test]
fn test_heuristic_token_counter_counts() {
    let response = HeuristicTokenCounter::new()
        .count_tokens(CountTokensRequest {
            text: "hello world",
        })
        .expect("count_tokens should succeed");
    assert!(response.count >= 2);
}

/// @covers: HeuristicTokenCounter::with_ratio — custom ratio drives estimate
#[test]
fn test_heuristic_token_counter_custom_ratio() {
    let response = HeuristicTokenCounter::with_ratio(4)
        .estimate_tokens(EstimateTokensRequest { text: "abcd" })
        .expect("estimate_tokens should succeed");
    assert_eq!(response.count, 1);
}

/// @covers: HeuristicTokenCounter::with_ratio — zero ratio clamps to one
#[test]
fn test_heuristic_token_counter_zero_ratio_clamped() {
    // Without clamping this would panic on divide-by-zero.
    let response = HeuristicTokenCounter::with_ratio(0)
        .estimate_tokens(EstimateTokensRequest { text: "ab" })
        .expect("estimate_tokens should succeed");
    assert_eq!(response.count, 2);
}

/// @covers: HeuristicTokenCounter — reports it is not an exact tokenizer
#[test]
fn test_heuristic_token_counter_not_exact() {
    let response = HeuristicTokenCounter::new()
        .is_exact(ExactnessRequest)
        .expect("is_exact should succeed");
    assert!(!response.exact);
}
