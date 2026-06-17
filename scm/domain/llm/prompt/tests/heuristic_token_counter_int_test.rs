//! Tests for the `HeuristicTokenCounter` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{HeuristicTokenCounter, TokenCounter};

/// @covers: HeuristicTokenCounter::new — counts tokens in text
#[test]
fn test_heuristic_token_counter_counts() {
    assert!(HeuristicTokenCounter::new().count_tokens("hello world") >= 2);
}

/// @covers: HeuristicTokenCounter::with_ratio — custom ratio drives estimate
#[test]
fn test_heuristic_token_counter_custom_ratio() {
    assert_eq!(
        HeuristicTokenCounter::with_ratio(4).estimate_tokens("abcd"),
        1
    );
}

/// @covers: HeuristicTokenCounter::with_ratio — zero ratio clamps to one
#[test]
fn test_heuristic_token_counter_zero_ratio_clamped() {
    // Without clamping this would panic on divide-by-zero.
    assert_eq!(
        HeuristicTokenCounter::with_ratio(0).estimate_tokens("ab"),
        2
    );
}

/// @covers: HeuristicTokenCounter — reports it is not an exact tokenizer
#[test]
fn test_heuristic_token_counter_not_exact() {
    assert!(!HeuristicTokenCounter::new().is_exact());
}
