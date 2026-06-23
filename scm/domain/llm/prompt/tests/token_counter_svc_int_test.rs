//! SAF facade tests — `TokenCounter` trait via `HeuristicTokenCounter`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptBootstrap, StdPromptFactory, TokenCounter};

fn counter() -> impl TokenCounter {
    StdPromptFactory::token_counter()
}

// --- count_tokens ---

/// @covers: TokenCounter::count_tokens — counts a non-empty string
#[test]
fn test_count_tokens_non_empty_happy() {
    assert!(counter().count_tokens("hello world") >= 2);
}

/// @covers: TokenCounter::count_tokens — empty string counts zero
#[test]
fn test_count_tokens_empty_is_zero_error() {
    assert_eq!(counter().count_tokens(""), 0);
}

/// @covers: TokenCounter::count_tokens — longer text counts more
#[test]
fn test_count_tokens_scales_with_length_edge() {
    let c = counter();
    assert!(c.count_tokens("a longer sentence with several words") > c.count_tokens("a"));
}

// --- estimate_tokens ---

/// @covers: TokenCounter::estimate_tokens — estimates a non-empty string
#[test]
fn test_estimate_tokens_non_empty_happy() {
    assert!(counter().estimate_tokens("abcdefgh") >= 1);
}

/// @covers: TokenCounter::estimate_tokens — empty string estimates zero
#[test]
fn test_estimate_tokens_empty_is_zero_error() {
    assert_eq!(counter().estimate_tokens(""), 0);
}

/// @covers: TokenCounter::estimate_tokens — single char rounds up to one
#[test]
fn test_estimate_tokens_single_char_one_edge() {
    assert_eq!(counter().estimate_tokens("a"), 1);
}

// --- tokenizer_name ---

/// @covers: TokenCounter::tokenizer_name — reports a stable name
#[test]
fn test_tokenizer_name_reports_name_happy() {
    assert_eq!(counter().tokenizer_name(), "heuristic-chars");
}

/// @covers: TokenCounter::tokenizer_name — name is non-empty
#[test]
fn test_tokenizer_name_non_empty_error() {
    assert!(!counter().tokenizer_name().is_empty());
}

/// @covers: TokenCounter::tokenizer_name — stable across calls
#[test]
fn test_tokenizer_name_stable_edge() {
    let c = counter();
    assert_eq!(c.tokenizer_name(), "heuristic-chars", "tokenizer name should be stable and known");
}

// --- is_exact ---

/// @covers: TokenCounter::is_exact — heuristic counter is not exact
#[test]
fn test_is_exact_false_happy() {
    assert!(!counter().is_exact());
}

/// @covers: TokenCounter::is_exact — never claims exactness
#[test]
fn test_is_exact_not_true_error() {
    assert!(!counter().is_exact());
}

/// @covers: TokenCounter::is_exact — stable across calls
#[test]
fn test_is_exact_stable_edge() {
    let c = counter();
    assert_eq!(c.is_exact(), false, "heuristic counter should never claim exactness");
}
