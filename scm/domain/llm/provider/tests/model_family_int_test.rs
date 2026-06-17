//! Tests for the `ModelFamily` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ModelFamily;

/// @covers: ModelFamily — variants are distinct
#[test]
fn test_model_family_variants_distinct() {
    assert_ne!(ModelFamily::Anthropic, ModelFamily::OpenAI);
    assert_ne!(ModelFamily::Google, ModelFamily::OpenSource);
}

/// @covers: ModelFamily — equality
#[test]
fn test_model_family_equality() {
    assert_eq!(ModelFamily::Anthropic, ModelFamily::Anthropic);
}

/// @covers: ModelFamily — serde round-trip
#[test]
fn test_model_family_serde_roundtrip() {
    let json = serde_json::to_string(&ModelFamily::Google).expect("serialize");
    let back: ModelFamily = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, ModelFamily::Google);
}
