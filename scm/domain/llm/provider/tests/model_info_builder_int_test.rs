//! Tests for `ModelInfoBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ModelFamily, ModelInfoBuilder};

/// @covers: ModelInfoBuilder::build — fluent overrides apply
#[test]
fn test_model_info_builder_applies_overrides() {
    let info = ModelInfoBuilder::new()
        .id("gpt-4".to_string())
        .name("GPT-4".to_string())
        .family(ModelFamily::OpenAI)
        .context_window(128_000)
        .supports_vision(true)
        .training_cutoff("2024-04".to_string())
        .build();
    assert_eq!(info.id, "gpt-4");
    assert_eq!(info.family, ModelFamily::OpenAI);
    assert!(info.supports_vision);
    assert_eq!(info.training_cutoff.as_deref(), Some("2024-04"));
}

/// @covers: ModelInfoBuilder::default — Other family and empty identity
#[test]
fn test_model_info_builder_defaults() {
    let info = ModelInfoBuilder::new().build();
    assert_eq!(info.family, ModelFamily::Other);
    assert!(info.id.is_empty());
}
