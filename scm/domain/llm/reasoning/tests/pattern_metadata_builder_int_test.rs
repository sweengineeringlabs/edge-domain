//! Tests for the `PatternMetadataBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{PatternMetadataBuilder, ReasoningPattern};

/// @covers: PatternMetadataBuilder — builds with overrides
#[test]
fn test_pattern_metadata_builder_builds_with_overrides() {
    let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
        .max_depth(20)
        .max_tokens(4000)
        .timeout_secs(120)
        .build();
    assert_eq!(meta.max_depth, 20);
    assert_eq!(meta.max_tokens, 4000);
    assert_eq!(meta.timeout_secs, 120);
}

/// @covers: PatternMetadataBuilder — clamps min confidence
#[test]
fn test_pattern_metadata_builder_clamps_confidence() {
    let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
        .min_confidence(5.0)
        .build();
    assert!((meta.min_confidence - 1.0).abs() < 0.001);
}

/// @covers: PatternMetadataBuilder — dedupes tags
#[test]
fn test_pattern_metadata_builder_dedupes_tags() {
    let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
        .tag("a".to_string())
        .tag("a".to_string())
        .build();
    assert_eq!(meta.tags.len(), 1);
}
