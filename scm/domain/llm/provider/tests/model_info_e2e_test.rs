//! Tests for `ModelInfo`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ModelFamily, ModelInfo};

/// @covers: ModelInfo::new — sets core fields correctly
#[test]
fn test_new_sets_core_fields_happy() {
    let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 128_000);
    assert_eq!(info.id, "gpt-4");
    assert_eq!(info.context_window, 128_000);
    assert_eq!(info.family, ModelFamily::OpenAI);
}

/// @covers: ModelInfo::new — defaults capabilities to off
#[test]
fn test_new_defaults_capabilities_off_error() {
    let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 128_000);
    assert!(!info.supports_vision);
}

/// @covers: ModelInfo — serializes and deserializes correctly
#[test]
fn test_model_info_serde_roundtrip_edge() {
    let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 8192);
    let json = serde_json::to_string(&info).expect("serialize");
    let back: ModelInfo = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.id, "gpt-4");
}
