//! Tests for `TokenizerAccuracy`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::TokenizerAccuracy;

/// @covers: TokenizerAccuracy — variants are distinct
#[test]
fn test_tokenizer_accuracy_variants_distinct_happy() {
    assert_ne!(TokenizerAccuracy::Exact, TokenizerAccuracy::Approximate);
    assert_ne!(TokenizerAccuracy::Approximate, TokenizerAccuracy::Fallback);
}

/// @covers: TokenizerAccuracy — equality holds for same variant
#[test]
fn test_tokenizer_accuracy_equality_error() {
    assert_eq!(TokenizerAccuracy::Exact, TokenizerAccuracy::Exact);
}

/// @covers: TokenizerAccuracy — serializes and deserializes correctly
#[test]
fn test_tokenizer_accuracy_serde_roundtrip_edge() {
    let json = serde_json::to_string(&TokenizerAccuracy::Fallback).expect("serialize");
    let back: TokenizerAccuracy = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, TokenizerAccuracy::Fallback);
}
