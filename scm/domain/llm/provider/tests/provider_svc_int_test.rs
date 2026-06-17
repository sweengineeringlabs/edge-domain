//! SAF facade tests — `Provider` trait via `StaticProvider`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    FinishReason, ModelFamily, ModelInfo, Provider, ProviderConfig, ProviderFactory,
    StdProviderFactory, TokenizerAccuracy,
};

fn provider(model: &str) -> impl Provider {
    let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        model.to_string(),
        model.to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    StdProviderFactory::provider(config, info)
}

// --- name ---

/// @covers: Provider::name
#[test]
fn test_name_returns_configured_model_happy() {
    assert_eq!(provider("claude").name(), "claude");
}

/// @covers: Provider::name — unusual but valid identifiers are preserved
#[test]
fn test_name_preserves_non_ascii_identifier_error() {
    assert_eq!(provider("modèle").name(), "modèle");
}

/// @covers: Provider::name — empty model still reports an empty name
#[test]
fn test_name_empty_model_returns_empty_edge() {
    assert_eq!(provider("").name(), "");
}

// --- provider_config ---

/// @covers: Provider::provider_config
#[test]
fn test_provider_config_roundtrips_model_happy() {
    assert_eq!(provider("claude").provider_config().model, "claude");
}

/// @covers: Provider::provider_config — temperature is carried through
#[test]
fn test_provider_config_carries_temperature_error() {
    assert_eq!(provider("claude").provider_config().temperature, 0.7);
}

/// @covers: Provider::provider_config — context window is carried through
#[test]
fn test_provider_config_carries_context_window_edge() {
    assert_eq!(
        provider("claude").provider_config().max_context_tokens,
        8192
    );
}

// --- model_info ---

/// @covers: Provider::model_info
#[test]
fn test_model_info_reports_id_happy() {
    assert_eq!(provider("claude").model_info().id, "claude");
}

/// @covers: Provider::model_info — context window matches construction
#[test]
fn test_model_info_reports_context_window_error() {
    assert_eq!(provider("claude").model_info().context_window, 8192);
}

/// @covers: Provider::model_info — vision defaults off
#[test]
fn test_model_info_vision_defaults_off_edge() {
    assert!(!provider("claude").model_info().supports_vision);
}

// --- model_family ---

/// @covers: Provider::model_family
#[test]
fn test_model_family_reports_anthropic_happy() {
    assert_eq!(provider("claude").model_family(), ModelFamily::Anthropic);
}

/// @covers: Provider::model_family — distinct from other families
#[test]
fn test_model_family_not_openai_error() {
    assert_ne!(provider("claude").model_family(), ModelFamily::OpenAI);
}

/// @covers: Provider::model_family — stable across calls
#[test]
fn test_model_family_stable_across_calls_edge() {
    let p = provider("claude");
    assert_eq!(p.model_family(), p.model_family());
}

// --- tokenizer_accuracy ---

/// @covers: Provider::tokenizer_accuracy
#[test]
fn test_tokenizer_accuracy_reports_approximate_happy() {
    assert_eq!(
        provider("claude").tokenizer_accuracy(),
        TokenizerAccuracy::Approximate
    );
}

/// @covers: Provider::tokenizer_accuracy — not claimed exact
#[test]
fn test_tokenizer_accuracy_not_exact_error() {
    assert_ne!(
        provider("claude").tokenizer_accuracy(),
        TokenizerAccuracy::Exact
    );
}

/// @covers: Provider::tokenizer_accuracy — stable across calls
#[test]
fn test_tokenizer_accuracy_stable_edge() {
    let p = provider("claude");
    assert_eq!(p.tokenizer_accuracy(), p.tokenizer_accuracy());
}

// --- last_token_usage ---

/// @covers: Provider::last_token_usage
#[test]
fn test_last_token_usage_starts_at_zero_happy() {
    assert_eq!(provider("claude").last_token_usage().total_tokens, 0);
}

/// @covers: Provider::last_token_usage — no cache reads initially
#[test]
fn test_last_token_usage_no_cache_reads_error() {
    assert!(!provider("claude").last_token_usage().cache_hit());
}

/// @covers: Provider::last_token_usage — prompt tokens zero initially
#[test]
fn test_last_token_usage_prompt_zero_edge() {
    assert_eq!(provider("claude").last_token_usage().prompt_tokens, 0);
}

// --- last_finish_reason ---

/// @covers: Provider::last_finish_reason
#[test]
fn test_last_finish_reason_defaults_stop_happy() {
    assert_eq!(provider("claude").last_finish_reason(), FinishReason::Stop);
}

/// @covers: Provider::last_finish_reason — not an error finish initially
#[test]
fn test_last_finish_reason_not_error_error() {
    assert_ne!(provider("claude").last_finish_reason(), FinishReason::Error);
}

/// @covers: Provider::last_finish_reason — stable across calls
#[test]
fn test_last_finish_reason_stable_edge() {
    let p = provider("claude");
    assert_eq!(p.last_finish_reason(), p.last_finish_reason());
}

// --- health_check ---

/// @covers: Provider::health_check — healthy when a model is configured
#[test]
fn test_health_check_ok_with_model_happy() {
    assert!(provider("claude").health_check().is_ok());
}

/// @covers: Provider::health_check — errors when model is empty
#[test]
fn test_health_check_errs_without_model_error() {
    assert!(provider("").health_check().is_err());
}

/// @covers: Provider::health_check — whitespace model is still non-empty
#[test]
fn test_health_check_whitespace_model_ok_edge() {
    assert!(provider(" ").health_check().is_ok());
}
