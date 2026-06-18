//! SAF facade tests — `PromptFactory` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    ContextManager, Prompt, PromptFactory, PromptMetadata, StdPromptFactory,
    TokenCounter, VariableType,
};

// --- std_factory ---

/// @covers: PromptFactory::std_factory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let _f: StdPromptFactory = StdPromptFactory::std_factory();
}

/// @covers: PromptFactory::std_factory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdPromptFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: PromptFactory::std_factory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let _a = StdPromptFactory::std_factory();
    let _b = StdPromptFactory::std_factory();
}

// --- variable_builder ---

/// @covers: PromptFactory::variable_builder — builds with overrides
#[test]
fn test_variable_builder_overrides_happy() {
    let v = StdPromptFactory::variable_builder()
        .name("topic".to_string())
        .var_type(VariableType::Number)
        .build();
    assert_eq!(v.var_type, VariableType::Number);
}

/// @covers: PromptFactory::variable_builder — default is required
#[test]
fn test_variable_builder_default_required_error() {
    assert!(StdPromptFactory::variable_builder().build().required);
}

/// @covers: PromptFactory::variable_builder — default value makes it optional
#[test]
fn test_variable_builder_default_value_optional_edge() {
    let v = StdPromptFactory::variable_builder()
        .default_value(serde_json::json!("x"))
        .build();
    assert!(!v.required);
}

// --- prompt_metadata_builder ---

/// @covers: PromptFactory::prompt_metadata_builder — builds with overrides
#[test]
fn test_prompt_metadata_builder_overrides_happy() {
    let m = StdPromptFactory::prompt_metadata_builder()
        .id("t".to_string())
        .build();
    assert_eq!(m.id, "t");
}

/// @covers: PromptFactory::prompt_metadata_builder — defaults to empty variables
#[test]
fn test_prompt_metadata_builder_default_empty_vars_error() {
    assert!(StdPromptFactory::prompt_metadata_builder()
        .build()
        .variables
        .is_empty());
}

/// @covers: PromptFactory::prompt_metadata_builder — tags carried through
#[test]
fn test_prompt_metadata_builder_tags_edge() {
    let m = StdPromptFactory::prompt_metadata_builder()
        .tags(vec!["system".to_string()])
        .build();
    assert_eq!(m.tags, vec!["system".to_string()]);
}

// --- prompt_cache_builder ---

/// @covers: PromptFactory::prompt_cache_builder — builds with overrides
#[test]
fn test_prompt_cache_builder_overrides_happy() {
    let c = StdPromptFactory::prompt_cache_builder()
        .key("k".to_string())
        .token_count(7)
        .build();
    assert_eq!(c.token_count, 7);
}

/// @covers: PromptFactory::prompt_cache_builder — default hit count is zero
#[test]
fn test_prompt_cache_builder_default_hits_error() {
    assert_eq!(
        StdPromptFactory::prompt_cache_builder().build().hit_count,
        0
    );
}

/// @covers: PromptFactory::prompt_cache_builder — custom TTL is applied
#[test]
fn test_prompt_cache_builder_custom_ttl_edge() {
    let c = StdPromptFactory::prompt_cache_builder()
        .ttl_seconds(60)
        .build();
    assert_eq!(c.ttl_seconds, 60);
}

// --- prompt ---

/// @covers: PromptFactory::prompt — builds a usable prompt
#[test]
fn test_prompt_builds_with_metadata_happy() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    assert_eq!(
        StdPromptFactory::prompt("body".to_string(), m)
            .metadata()
            .id,
        "p"
    );
}

/// @covers: PromptFactory::prompt — unbalanced template fails validation
#[test]
fn test_prompt_unbalanced_template_invalid_error() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    assert!(StdPromptFactory::prompt("{{x}".to_string(), m)
        .validate()
        .is_err());
}

/// @covers: PromptFactory::prompt — empty template validates
#[test]
fn test_prompt_empty_template_valid_edge() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    assert!(StdPromptFactory::prompt(String::new(), m)
        .validate()
        .is_ok());
}

// --- context_manager ---

/// @covers: PromptFactory::context_manager — builds an empty manager
#[test]
fn test_context_manager_starts_complete_happy() {
    assert!(StdPromptFactory::context_manager().is_complete());
}

/// @covers: PromptFactory::context_manager — unknown variable is absent
#[test]
fn test_context_manager_no_variables_error() {
    assert!(StdPromptFactory::context_manager()
        .get_variable("x")
        .is_none());
}

/// @covers: PromptFactory::context_manager — independent instances per call
#[test]
fn test_context_manager_independent_instances_edge() {
    use edge_llm_prompt::Variable;
    let mut a = StdPromptFactory::context_manager();
    a.register_variable(
        "x".to_string(),
        Variable::new("x".to_string(), VariableType::String),
    )
    .expect("register");
    let b = StdPromptFactory::context_manager();
    assert!(b.get_variable("x").is_none());
}

// --- token_counter ---

/// @covers: PromptFactory::token_counter — builds a working counter
#[test]
fn test_token_counter_counts_text_happy() {
    assert!(StdPromptFactory::token_counter().count_tokens("hello") >= 1);
}

/// @covers: PromptFactory::token_counter — empty string counts zero
#[test]
fn test_token_counter_empty_zero_error() {
    assert_eq!(StdPromptFactory::token_counter().count_tokens(""), 0);
}

/// @covers: PromptFactory::token_counter — reports it is not exact
#[test]
fn test_token_counter_not_exact_edge() {
    assert!(!StdPromptFactory::token_counter().is_exact());
}

// --- default_prompt_handler ---

/// @covers: default_prompt_handler — builds a usable Handler
#[test]
fn test_default_prompt_handler_renders_happy() {
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    let var = Variable::new("name".to_string(), VariableType::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let h = StdPromptFactory::default_prompt_handler("Hi {{name}}".to_string(), m);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let render_ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(Handler::execute(&h, render_ctx, ctx)).expect("ok");
    assert_eq!(out, "Hi Ada");
}

/// @covers: default_prompt_handler — missing required variable surfaces an error through the pipeline
#[test]
fn test_default_prompt_handler_missing_variable_errors_error() {
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    let var = Variable::new("name".to_string(), VariableType::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let h = StdPromptFactory::default_prompt_handler("Hi {{name}}".to_string(), m);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    assert!(block_on(Handler::execute(&h, RenderContext::new(), ctx)).is_err());
}

/// @covers: default_prompt_handler — exposes the stable dispatch id
#[test]
fn test_default_prompt_handler_id_is_stable_edge() {
    use edge_domain_handler::Handler;
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![]);
    let h = StdPromptFactory::default_prompt_handler("static".to_string(), m);
    assert_eq!(Handler::id(&h), "prompt.render");
}

// --- prompt_handler ---

/// @covers: StdPromptFactory::prompt_handler — builds a usable Handler from a prompt impl
#[test]
fn test_prompt_handler_renders_with_arc_prompt_happy() {
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    use std::sync::Arc;
    let var = Variable::new("name".to_string(), VariableType::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let prompt = Arc::new(StdPromptFactory::prompt("Hi {{name}}".to_string(), m));
    let h = StdPromptFactory::prompt_handler(prompt);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let render_ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Eve"));
    let out = block_on(Handler::execute(&h, render_ctx, ctx)).expect("ok");
    assert_eq!(out, "Hi Eve");
}

/// @covers: StdPromptFactory::prompt_handler — missing required variable is an error
#[test]
fn test_prompt_handler_missing_required_variable_error() {
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    use std::sync::Arc;
    let var = Variable::new("name".to_string(), VariableType::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let prompt = Arc::new(StdPromptFactory::prompt("Hi {{name}}".to_string(), m));
    let h = StdPromptFactory::prompt_handler(prompt);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    assert!(block_on(Handler::execute(&h, RenderContext::new(), ctx)).is_err());
}

/// @covers: StdPromptFactory::prompt_handler — empty template renders to an empty string
#[test]
fn test_prompt_handler_empty_template_edge() {
    use edge_domain_handler::Handler;
    use std::sync::Arc;
    let m = PromptMetadata::new("e".to_string(), "E".to_string(), "1".to_string(), vec![]);
    let prompt = Arc::new(StdPromptFactory::prompt(String::new(), m));
    let h = StdPromptFactory::prompt_handler(prompt);
    assert_eq!(Handler::id(&h), "prompt.render");
}
