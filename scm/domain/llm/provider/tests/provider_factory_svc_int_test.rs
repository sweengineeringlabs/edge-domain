//! SAF facade tests — `ProviderFactory` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    ExecutionConfig, ExecutionMode, ExecutionModel, ModelFamily, ModelInfo, Provider,
    ProviderConfig, ProviderFactory, StdProviderFactory, StreamHandler,
};

// --- execution_config_builder ---

/// @covers: ProviderFactory::execution_config_builder — builds with overrides
#[test]
fn test_execution_config_builder_overrides_happy() {
    let config = StdProviderFactory::execution_config_builder()
        .max_tokens_per_call(2048)
        .build();
    assert_eq!(config.max_tokens_per_call, 2048);
}

/// @covers: ProviderFactory::execution_config_builder — default mode is async
#[test]
fn test_execution_config_builder_default_mode_error() {
    let config = StdProviderFactory::execution_config_builder().build();
    assert_eq!(config.execution_mode, ExecutionMode::Async);
}

/// @covers: ProviderFactory::execution_config_builder — streaming requires the flag
#[test]
fn test_execution_config_builder_streaming_flag_edge() {
    let config = StdProviderFactory::execution_config_builder()
        .execution_mode(ExecutionMode::Streaming)
        .build();
    assert!(!config.supports_streaming());
}

// --- provider_config_builder ---

/// @covers: ProviderFactory::provider_config_builder — builds with overrides
#[test]
fn test_provider_config_builder_overrides_happy() {
    let config = StdProviderFactory::provider_config_builder()
        .model("claude".to_string())
        .build();
    assert_eq!(config.model, "claude");
}

/// @covers: ProviderFactory::provider_config_builder — empty model by default
#[test]
fn test_provider_config_builder_default_empty_error() {
    assert!(StdProviderFactory::provider_config_builder()
        .build()
        .model
        .is_empty());
}

/// @covers: ProviderFactory::provider_config_builder — api base is optional
#[test]
fn test_provider_config_builder_api_base_optional_edge() {
    assert!(StdProviderFactory::provider_config_builder()
        .build()
        .api_base
        .is_none());
}

// --- model_info_builder ---

/// @covers: ProviderFactory::model_info_builder — builds with overrides
#[test]
fn test_model_info_builder_overrides_happy() {
    let info = StdProviderFactory::model_info_builder()
        .id("gpt-4".to_string())
        .family(ModelFamily::OpenAI)
        .build();
    assert_eq!(info.family, ModelFamily::OpenAI);
}

/// @covers: ProviderFactory::model_info_builder — defaults to the Other family
#[test]
fn test_model_info_builder_default_family_error() {
    assert_eq!(
        StdProviderFactory::model_info_builder().build().family,
        ModelFamily::Other
    );
}

/// @covers: ProviderFactory::model_info_builder — training cutoff optional
#[test]
fn test_model_info_builder_cutoff_optional_edge() {
    assert!(StdProviderFactory::model_info_builder()
        .build()
        .training_cutoff
        .is_none());
}

// --- token_usage_builder ---

/// @covers: ProviderFactory::token_usage_builder — totals are computed
#[test]
fn test_token_usage_builder_totals_happy() {
    let usage = StdProviderFactory::token_usage_builder()
        .prompt_tokens(10)
        .completion_tokens(5)
        .build();
    assert_eq!(usage.total_tokens, 15);
}

/// @covers: ProviderFactory::token_usage_builder — defaults to zero
#[test]
fn test_token_usage_builder_default_zero_error() {
    assert_eq!(
        StdProviderFactory::token_usage_builder()
            .build()
            .total_tokens,
        0
    );
}

/// @covers: ProviderFactory::token_usage_builder — cache reads flip cache_hit
#[test]
fn test_token_usage_builder_cache_hit_edge() {
    let usage = StdProviderFactory::token_usage_builder()
        .cache_read_input_tokens(5)
        .build();
    assert!(usage.cache_hit());
}

// --- tool_call_delta_builder ---

/// @covers: ProviderFactory::tool_call_delta_builder — sets the index
#[test]
fn test_tool_call_delta_builder_index_happy() {
    assert_eq!(
        StdProviderFactory::tool_call_delta_builder(3).build().index,
        3
    );
}

/// @covers: ProviderFactory::tool_call_delta_builder — name optional by default
#[test]
fn test_tool_call_delta_builder_name_optional_error() {
    assert!(StdProviderFactory::tool_call_delta_builder(0)
        .build()
        .name
        .is_none());
}

/// @covers: ProviderFactory::tool_call_delta_builder — overrides apply
#[test]
fn test_tool_call_delta_builder_overrides_edge() {
    let delta = StdProviderFactory::tool_call_delta_builder(1)
        .name("search".to_string())
        .build();
    assert_eq!(delta.name.as_deref(), Some("search"));
}

// --- std_factory ---

/// @covers: ProviderFactory::std_factory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let _f: StdProviderFactory = StdProviderFactory::std_factory();
}

/// @covers: ProviderFactory::std_factory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdProviderFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: ProviderFactory::std_factory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let _a = StdProviderFactory::std_factory();
    let _b = StdProviderFactory::std_factory();
}

// --- provider ---

/// @covers: ProviderFactory::provider — builds a usable provider
#[test]
fn test_provider_builds_named_provider_happy() {
    let config = ProviderConfig::new("claude".to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    assert_eq!(StdProviderFactory::provider(config, info).name(), "claude");
}

/// @covers: ProviderFactory::provider — empty model produces an unhealthy provider
#[test]
fn test_provider_empty_model_unhealthy_error() {
    let config = ProviderConfig::new(String::new(), 0.7, 8192);
    let info = ModelInfo::new(String::new(), String::new(), ModelFamily::OpenAI, 8192);
    assert!(StdProviderFactory::provider(config, info)
        .health_check()
        .is_err());
}

/// @covers: ProviderFactory::provider — family flows from model metadata
#[test]
fn test_provider_reports_model_family_edge() {
    let config = ProviderConfig::new("gpt".to_string(), 0.5, 4096);
    let info = ModelInfo::new(
        "gpt".to_string(),
        "GPT".to_string(),
        ModelFamily::OpenAI,
        4096,
    );
    assert_eq!(
        StdProviderFactory::provider(config, info).model_family(),
        ModelFamily::OpenAI
    );
}

// --- execution_model ---

/// @covers: ProviderFactory::execution_model — builds a model in the given mode
#[test]
fn test_execution_model_builds_in_mode_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    assert_eq!(
        StdProviderFactory::execution_model(config).execution_mode(),
        ExecutionMode::Async
    );
}

/// @covers: ProviderFactory::execution_model — zero budget cannot execute
#[test]
fn test_execution_model_zero_budget_blocked_error() {
    let config = ExecutionConfig::new(0, 30_000, true, false, ExecutionMode::Async);
    assert!(StdProviderFactory::execution_model(config)
        .can_execute()
        .is_err());
}

/// @covers: ProviderFactory::execution_model — streaming mode preserved
#[test]
fn test_execution_model_streaming_mode_edge() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
    assert_eq!(
        StdProviderFactory::execution_model(config).execution_mode(),
        ExecutionMode::Streaming
    );
}

// --- stream_handler ---

/// @covers: ProviderFactory::stream_handler — builds an empty handler
#[test]
fn test_stream_handler_starts_empty_happy() {
    let mut h = StdProviderFactory::stream_handler();
    assert!(h.next_chunk().is_none());
}

/// @covers: ProviderFactory::stream_handler — no pending tool call initially
#[test]
fn test_stream_handler_no_pending_call_error() {
    let h = StdProviderFactory::stream_handler();
    assert!(h.pending_tool_call().is_none());
}

/// @covers: ProviderFactory::stream_handler — independent instances per call
#[test]
fn test_stream_handler_independent_instances_edge() {
    let mut a = StdProviderFactory::stream_handler();
    a.accumulate(edge_llm_provider::StreamDelta::text("x".to_string()));
    let mut b = StdProviderFactory::stream_handler();
    assert!(b.next_chunk().is_none());
}
