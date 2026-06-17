//! Tests for the `ModelInfo` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ModelFamily, ModelInfo};

/// @covers: ModelInfo::new — sets identity and context window
#[test]
fn test_new_sets_core_fields() {
    let info = ModelInfo::new(
        "gpt-4".to_string(),
        "GPT-4".to_string(),
        ModelFamily::OpenAI,
        128_000,
    );
    assert_eq!(info.id, "gpt-4");
    assert_eq!(info.context_window, 128_000);
    assert_eq!(info.family, ModelFamily::OpenAI);
}

/// @covers: ModelInfo::new — capability flags default off
#[test]
fn test_new_defaults_capabilities_off() {
    let info = ModelInfo::new(
        "gpt-4".to_string(),
        "GPT-4".to_string(),
        ModelFamily::OpenAI,
        128_000,
    );
    assert!(!info.supports_vision);
}

/// @covers: ModelInfo — serde round-trip
#[test]
fn test_model_info_serde_roundtrip() {
    let info = ModelInfo::new(
        "gpt-4".to_string(),
        "GPT-4".to_string(),
        ModelFamily::OpenAI,
        8192,
    );
    let json = serde_json::to_string(&info).expect("serialize");
    let back: ModelInfo = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.id, "gpt-4");
}
