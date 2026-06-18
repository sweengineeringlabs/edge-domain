//! Tests for `ModelInfoBuilder` setter methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ModelFamily, ModelInfoBuilder};

fn mk() -> ModelInfoBuilder {
    ModelInfoBuilder::new().id("x".to_string()).family(ModelFamily::Anthropic)
}

/// @covers: ModelInfoBuilder::id — sets the model identifier
#[test]
fn test_id() {
    let info = ModelInfoBuilder::new().id("claude-3".to_string()).build();
    assert_eq!(info.id, "claude-3");
}

/// @covers: ModelInfoBuilder::id — empty string is preserved
#[test]
fn test_id_empty_edge() {
    let info = ModelInfoBuilder::new().id(String::new()).build();
    assert_eq!(info.id, "");
}

/// @covers: ModelInfoBuilder::name — sets the display name
#[test]
fn test_name() {
    let info = mk().name("Claude".to_string()).build();
    assert_eq!(info.name, "Claude");
}

/// @covers: ModelInfoBuilder::family — sets the model family
#[test]
fn test_family() {
    let info = ModelInfoBuilder::new().family(ModelFamily::OpenAI).build();
    assert_eq!(info.family, ModelFamily::OpenAI);
}

/// @covers: ModelInfoBuilder::context_window — sets the context window size
#[test]
fn test_context_window() {
    let info = mk().context_window(200_000).build();
    assert_eq!(info.context_window, 200_000);
}

/// @covers: ModelInfoBuilder::context_window — zero is the minimum
#[test]
fn test_context_window_zero_edge() {
    let info = mk().context_window(0).build();
    assert_eq!(info.context_window, 0);
}

/// @covers: ModelInfoBuilder::supports_vision — sets the vision flag
#[test]
fn test_supports_vision() {
    let info = mk().supports_vision(true).build();
    assert!(info.supports_vision);
}

/// @covers: ModelInfoBuilder::supports_functions — sets the functions flag
#[test]
fn test_supports_functions() {
    let info = mk().supports_functions(true).build();
    assert!(info.supports_functions);
}

/// @covers: ModelInfoBuilder::supports_streaming — sets the streaming flag
#[test]
fn test_supports_streaming() {
    let info = mk().supports_streaming(true).build();
    assert!(info.supports_streaming);
}

/// @covers: ModelInfoBuilder::training_cutoff — sets the training cutoff date
#[test]
fn test_training_cutoff() {
    let info = mk().training_cutoff("2024-01".to_string()).build();
    assert_eq!(info.training_cutoff.as_deref(), Some("2024-01"));
}

/// @covers: ModelInfoBuilder::build — produces a ModelInfo with defaults
#[test]
fn test_build() {
    let info = ModelInfoBuilder::new().build();
    assert!(!info.supports_vision);
    assert!(!info.supports_functions);
}

/// @covers: ModelInfoBuilder::build — capability flags default to false
#[test]
fn test_build_flags_default_false_edge() {
    let info = ModelInfoBuilder::new().build();
    assert!(!info.supports_streaming);
}
