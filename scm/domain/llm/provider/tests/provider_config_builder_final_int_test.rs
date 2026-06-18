//! Tests for `ProviderConfigBuilder` setter methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ProviderConfigBuilder;

/// @covers: ProviderConfigBuilder::model — sets the model name
#[test]
fn test_model() {
    let c = ProviderConfigBuilder::new().model("gpt-4".to_string()).build();
    assert_eq!(c.model, "gpt-4");
}

/// @covers: ProviderConfigBuilder::model — empty model name is preserved
#[test]
fn test_model_empty_edge() {
    let c = ProviderConfigBuilder::new().model(String::new()).build();
    assert_eq!(c.model, "");
}

/// @covers: ProviderConfigBuilder::temperature — sets the temperature
#[test]
fn test_temperature() {
    let c = ProviderConfigBuilder::new().temperature(0.5).build();
    assert_eq!(c.temperature, 0.5);
}

/// @covers: ProviderConfigBuilder::api_base — sets the base URL
#[test]
fn test_api_base() {
    let c = ProviderConfigBuilder::new().api_base("https://api.example.com".to_string()).build();
    assert_eq!(c.api_base.as_deref(), Some("https://api.example.com"));
}

/// @covers: ProviderConfigBuilder::api_base — absent by default
#[test]
fn test_api_base_absent_by_default_edge() {
    let c = ProviderConfigBuilder::new().build();
    assert!(c.api_base.is_none());
}

/// @covers: ProviderConfigBuilder::max_context_tokens — sets the context window
#[test]
fn test_max_context_tokens() {
    let c = ProviderConfigBuilder::new().max_context_tokens(16384).build();
    assert_eq!(c.max_context_tokens, 16384);
}

/// @covers: ProviderConfigBuilder::supports_vision — sets the vision flag
#[test]
fn test_supports_vision() {
    let c = ProviderConfigBuilder::new().supports_vision(true).build();
    assert!(c.supports_vision);
}

/// @covers: ProviderConfigBuilder::supports_functions — sets the functions flag
#[test]
fn test_supports_functions() {
    let c = ProviderConfigBuilder::new().supports_functions(true).build();
    assert!(c.supports_functions);
}

/// @covers: ProviderConfigBuilder::supports_streaming — sets the streaming flag
#[test]
fn test_supports_streaming() {
    let c = ProviderConfigBuilder::new().supports_streaming(true).build();
    assert!(c.supports_streaming);
}

/// @covers: ProviderConfigBuilder::build — all capability flags default to false
#[test]
fn test_build() {
    let c = ProviderConfigBuilder::new().build();
    assert!(!c.supports_vision);
    assert!(!c.supports_functions);
}

/// @covers: ProviderConfigBuilder::build — streaming defaults to false
#[test]
fn test_build_streaming_default_false_edge() {
    let c = ProviderConfigBuilder::new().build();
    assert!(!c.supports_streaming);
}
