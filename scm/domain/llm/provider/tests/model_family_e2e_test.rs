//! Tests for `ModelFamily`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ModelFamily;

/// @covers: ModelFamily — variants are distinct
#[test]
fn test_model_family_variants_distinct_happy() {
    assert_ne!(ModelFamily::Anthropic, ModelFamily::OpenAI);
    assert_ne!(ModelFamily::Google, ModelFamily::OpenSource);
}

/// @covers: ModelFamily — equality holds for same variant
#[test]
fn test_model_family_equality_error() {
    assert_eq!(ModelFamily::Anthropic, ModelFamily::Anthropic);
}

/// @covers: ModelFamily — serializes and deserializes correctly
#[test]
fn test_model_family_serde_roundtrip_edge() {
    let json = serde_json::to_string(&ModelFamily::Google).expect("serialize");
    let back: ModelFamily = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, ModelFamily::Google);
}
