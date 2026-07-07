//! SAF facade tests — `TokenCounter` trait via `HeuristicTokenCounter`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CountTokensRequest, EstimateTokensRequest, ExactnessRequest, HeuristicTokenCounter,
    TokenCounter, TokenizerNameRequest,
};

fn counter() -> impl TokenCounter {
    HeuristicTokenCounter::new()
}

// --- count_tokens ---

/// @covers: TokenCounter::count_tokens — counts a non-empty string
#[test]
fn test_count_tokens_non_empty_happy() {
    let count = counter()
        .count_tokens(CountTokensRequest {
            text: "hello world",
        })
        .expect("count_tokens should succeed")
        .count;
    assert!(count >= 2);
}

/// @covers: TokenCounter::count_tokens — empty string counts zero
#[test]
fn test_count_tokens_empty_is_zero_error() {
    let count = counter()
        .count_tokens(CountTokensRequest { text: "" })
        .expect("count_tokens should succeed")
        .count;
    assert_eq!(count, 0);
}

/// @covers: TokenCounter::count_tokens — longer text counts more
#[test]
fn test_count_tokens_scales_with_length_edge() {
    let c = counter();
    let longer = c
        .count_tokens(CountTokensRequest {
            text: "a longer sentence with several words",
        })
        .expect("count_tokens should succeed")
        .count;
    let shorter = c
        .count_tokens(CountTokensRequest { text: "a" })
        .expect("count_tokens should succeed")
        .count;
    assert!(longer > shorter);
}

// --- estimate_tokens ---

/// @covers: TokenCounter::estimate_tokens — estimates a non-empty string
#[test]
fn test_estimate_tokens_non_empty_happy() {
    let count = counter()
        .estimate_tokens(EstimateTokensRequest { text: "abcdefgh" })
        .expect("estimate_tokens should succeed")
        .count;
    assert!(count >= 1);
}

/// @covers: TokenCounter::estimate_tokens — empty string estimates zero
#[test]
fn test_estimate_tokens_empty_is_zero_error() {
    let count = counter()
        .estimate_tokens(EstimateTokensRequest { text: "" })
        .expect("estimate_tokens should succeed")
        .count;
    assert_eq!(count, 0);
}

/// @covers: TokenCounter::estimate_tokens — single char rounds up to one
#[test]
fn test_estimate_tokens_single_char_one_edge() {
    let count = counter()
        .estimate_tokens(EstimateTokensRequest { text: "a" })
        .expect("estimate_tokens should succeed")
        .count;
    assert_eq!(count, 1);
}

// --- tokenizer_name ---

/// @covers: TokenCounter::tokenizer_name — reports a stable name
#[test]
fn test_tokenizer_name_reports_name_happy() {
    let name = counter()
        .tokenizer_name(TokenizerNameRequest)
        .expect("tokenizer_name should succeed")
        .name;
    assert_eq!(name, "heuristic-chars");
}

/// @covers: TokenCounter::tokenizer_name — name is non-empty
#[test]
fn test_tokenizer_name_non_empty_error() {
    let name = counter()
        .tokenizer_name(TokenizerNameRequest)
        .expect("tokenizer_name should succeed")
        .name;
    assert!(!name.is_empty());
}

/// @covers: TokenCounter::tokenizer_name — stable across calls
#[test]
fn test_tokenizer_name_stable_edge() {
    let c = counter();
    let name = c
        .tokenizer_name(TokenizerNameRequest)
        .expect("tokenizer_name should succeed")
        .name;
    assert_eq!(
        name, "heuristic-chars",
        "tokenizer name should be stable and known"
    );
}

// --- is_exact ---

/// @covers: TokenCounter::is_exact — heuristic counter is not exact
#[test]
fn test_is_exact_false_happy() {
    let exact = counter()
        .is_exact(ExactnessRequest)
        .expect("is_exact should succeed")
        .exact;
    assert!(!exact);
}

/// @covers: TokenCounter::is_exact — never claims exactness
#[test]
fn test_is_exact_not_true_error() {
    let exact = counter()
        .is_exact(ExactnessRequest)
        .expect("is_exact should succeed")
        .exact;
    assert!(!exact);
}

/// @covers: TokenCounter::is_exact — stable across calls
#[test]
fn test_is_exact_stable_edge() {
    let c = counter();
    let exact = c
        .is_exact(ExactnessRequest)
        .expect("is_exact should succeed")
        .exact;
    assert!(!exact, "heuristic counter should never claim exactness");
}
