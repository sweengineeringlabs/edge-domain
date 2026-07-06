//! End-to-end contract tests for the `PromptBootstrap` trait, exercised
//! through the crate's reference implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptBootstrap, PromptBootstrapNameRequest, StdPromptFactory};

/// @covers: PromptBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string() {
    let f = StdPromptFactory;
    let name = f
        .bootstrap_name(PromptBootstrapNameRequest)
        .expect("bootstrap_name ok")
        .name;
    assert!(!name.is_empty());
}

/// @covers: PromptBootstrap::std_factory
#[test]
fn test_std_factory_returns_zero_sized_instance() {
    let f: StdPromptFactory = StdPromptFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: PromptBootstrap::variable_builder
#[test]
fn test_variable_builder_starts_required() {
    assert!(StdPromptFactory::variable_builder().build().required);
}

/// @covers: PromptBootstrap::prompt_metadata_builder
#[test]
fn test_prompt_metadata_builder_starts_empty_variables() {
    assert!(StdPromptFactory::prompt_metadata_builder()
        .build()
        .variables
        .is_empty());
}

/// @covers: PromptBootstrap::prompt_cache_builder
#[test]
fn test_prompt_cache_builder_default_ttl_is_one_hour() {
    assert_eq!(
        StdPromptFactory::prompt_cache_builder().build().ttl_seconds,
        3600
    );
}

/// @covers: PromptBootstrap::prompt_cache
#[test]
fn test_prompt_cache_constructs_entry_directly() {
    let c = StdPromptFactory::prompt_cache("k".into(), "r".into(), 3);
    assert_eq!(c.key, "k");
}

/// @covers: PromptBootstrap::prompt
#[test]
fn test_prompt_builds_static_prompt() {
    use edge_llm_prompt::{Prompt, PromptMetadata, PromptMetadataRequest};
    let m = PromptMetadata::new("p".into(), "P".into(), "1".into(), vec![]);
    let prompt = StdPromptFactory::prompt("body".into(), m);
    assert_eq!(prompt.metadata(PromptMetadataRequest).expect("ok").id, "p");
}

/// @covers: PromptBootstrap::context_manager
#[test]
fn test_context_manager_starts_complete() {
    use edge_llm_prompt::{CompletenessRequest, ContextManager};
    assert!(
        StdPromptFactory::context_manager()
            .is_complete(CompletenessRequest)
            .expect("ok")
            .complete
    );
}

/// @covers: PromptBootstrap::token_counter
#[test]
fn test_token_counter_reports_inexact() {
    use edge_llm_prompt::{ExactnessRequest, TokenCounter};
    assert!(
        !StdPromptFactory::token_counter()
            .is_exact(ExactnessRequest)
            .expect("ok")
            .exact
    );
}

/// @covers: PromptBootstrap::template_provider
#[test]
fn test_template_provider_starts_empty() {
    use edge_llm_prompt::{ListTemplatesRequest, TemplateProvider};
    assert!(StdPromptFactory::template_provider()
        .list_templates(ListTemplatesRequest)
        .expect("ok")
        .templates
        .is_empty());
}

/// @covers: PromptBootstrap::prompt_template_builder
#[test]
fn test_prompt_template_builder_starts_empty_id() {
    assert!(StdPromptFactory::prompt_template_builder()
        .build()
        .id
        .is_empty());
}
