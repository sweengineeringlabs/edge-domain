//! Tests for the `PatternMetadata` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{PatternMetadata, ReasoningPattern};

/// @covers: PatternMetadata::new — seeds defaults from the pattern
#[test]
fn test_new_seeds_pattern() {
    let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought);
    assert_eq!(meta.pattern, ReasoningPattern::ChainOfThought);
    assert_eq!(meta.max_tokens, 8000);
}

/// @covers: PatternMetadata::new — iterative patterns allow backtracking
#[test]
fn test_new_iterative_allows_backtracking() {
    assert!(PatternMetadata::new(ReasoningPattern::Reflection).allow_backtracking);
}

/// @covers: PatternMetadata::with_min_confidence — clamps above one
#[test]
fn test_with_min_confidence_clamps_high() {
    let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought).with_min_confidence(2.0);
    assert!((meta.min_confidence - 1.0).abs() < 0.001);
}

/// @covers: PatternMetadata::with_tag — dedupes tags
#[test]
fn test_with_tag_dedupes() {
    let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought)
        .with_tag("a".to_string())
        .with_tag("a".to_string());
    assert_eq!(meta.tags.len(), 1);
}

/// @covers: PatternMetadata — serde round-trip
#[test]
fn test_pattern_metadata_serde_roundtrip() {
    let meta = PatternMetadata::new(ReasoningPattern::TreeOfThought);
    let json = serde_json::to_string(&meta).expect("serialize");
    let back: PatternMetadata = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.pattern, ReasoningPattern::TreeOfThought);
}
