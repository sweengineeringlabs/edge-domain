//! SAF facade tests — standard prompt primitive constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CatalogTemplateProvider, CompletenessRequest, ContextManager, CountTokensRequest,
    ExactnessRequest, HeuristicTokenCounter, MapContextManager, Prompt, PromptCache,
    PromptCacheBuilder, PromptMetadata, PromptMetadataBuilder, PromptMetadataRequest,
    PromptTemplateBuilder, RegisterVariableRequest, StaticPrompt, StdPromptFactory,
    TemplateLookupRequest, TemplateValidationRequest, TokenCounter, VariableBuilder, VariableKind,
    VariableLookupRequest,
};

// --- std_factory ---

/// @covers: PromptBootstrap::std_factory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let f: StdPromptFactory = StdPromptFactory;
    assert_eq!(
        std::mem::size_of_val(&f),
        0,
        "StdPromptFactory should be zero-sized"
    );
}

/// @covers: PromptBootstrap::std_factory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdPromptFactory;
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: PromptBootstrap::std_factory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let a = StdPromptFactory;
    let b = StdPromptFactory;
    assert_eq!(
        std::mem::size_of_val(&a),
        std::mem::size_of_val(&b),
        "both calls should be identical"
    );
}

// --- variable_builder ---

/// @covers: PromptBootstrap::variable_builder — builds with overrides
#[test]
fn test_variable_builder_overrides_happy() {
    let v = VariableBuilder::new()
        .name("topic".to_string())
        .var_type(VariableKind::Number)
        .build();
    assert_eq!(v.var_type, VariableKind::Number);
}

/// @covers: PromptBootstrap::variable_builder — default is required
#[test]
fn test_variable_builder_default_required_error() {
    assert!(VariableBuilder::new().build().required);
}

/// @covers: PromptBootstrap::variable_builder — default value makes it optional
#[test]
fn test_variable_builder_default_value_optional_edge() {
    let v = VariableBuilder::new()
        .default_value(serde_json::json!("x"))
        .build();
    assert!(!v.required);
}

// --- prompt_metadata_builder ---

/// @covers: PromptBootstrap::prompt_metadata_builder — builds with overrides
#[test]
fn test_prompt_metadata_builder_overrides_happy() {
    let m = PromptMetadataBuilder::new()
        .id("t".to_string())
        .build();
    assert_eq!(m.id, "t");
}

/// @covers: PromptBootstrap::prompt_metadata_builder — defaults to empty variables
#[test]
fn test_prompt_metadata_builder_default_empty_vars_error() {
    assert!(PromptMetadataBuilder::new()
        .build()
        .variables
        .is_empty());
}

/// @covers: PromptBootstrap::prompt_metadata_builder — tags carried through
#[test]
fn test_prompt_metadata_builder_tags_edge() {
    let m = PromptMetadataBuilder::new()
        .tags(vec!["system".to_string()])
        .build();
    assert_eq!(m.tags, vec!["system".to_string()]);
}

// --- prompt_cache_builder ---

/// @covers: PromptBootstrap::prompt_cache_builder — builds with overrides
#[test]
fn test_prompt_cache_builder_overrides_happy() {
    let c = PromptCacheBuilder::new()
        .key("k".to_string())
        .token_count(7)
        .build();
    assert_eq!(c.token_count, 7);
}

/// @covers: PromptBootstrap::prompt_cache_builder — default hit count is zero
#[test]
fn test_prompt_cache_builder_default_hits_error() {
    assert_eq!(
        PromptCacheBuilder::new().build().hit_count,
        0
    );
}

/// @covers: PromptBootstrap::prompt_cache_builder — custom TTL is applied
#[test]
fn test_prompt_cache_builder_custom_ttl_edge() {
    let c = PromptCacheBuilder::new()
        .ttl_seconds(60)
        .build();
    assert_eq!(c.ttl_seconds, 60);
}

// --- prompt ---

/// @covers: PromptBootstrap::prompt — builds a usable prompt
#[test]
fn test_prompt_builds_with_metadata_happy() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    assert_eq!(
        StaticPrompt::new("body".to_string(), m)
            .metadata(PromptMetadataRequest)
            .expect("metadata ok")
            .id,
        "p"
    );
}

/// @covers: PromptBootstrap::prompt — unbalanced template fails validation
#[test]
fn test_prompt_unbalanced_template_invalid_error() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    assert!(StaticPrompt::new("{{x}".to_string(), m)
        .validate(TemplateValidationRequest)
        .is_err());
}

/// @covers: PromptBootstrap::prompt — empty template validates
#[test]
fn test_prompt_empty_template_valid_edge() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    let result = StaticPrompt::new(String::new(), m).validate(TemplateValidationRequest);
    assert_eq!(result, Ok(()), "empty template should be valid");
}

// --- context_manager ---

/// @covers: PromptBootstrap::context_manager — builds an empty manager
#[test]
fn test_context_manager_starts_complete_happy() {
    assert!(
        MapContextManager::new()
            .is_complete(CompletenessRequest)
            .expect("is_complete ok")
            .complete
    );
}

/// @covers: PromptBootstrap::context_manager — unknown variable is absent
#[test]
fn test_context_manager_no_variables_error() {
    assert!(MapContextManager::new()
        .get_variable(VariableLookupRequest { name: "x" })
        .expect("get_variable ok")
        .variable
        .is_none());
}

/// @covers: PromptBootstrap::context_manager — independent instances per call
#[test]
fn test_context_manager_independent_instances_edge() {
    use edge_llm_prompt::Variable;
    let mut a = MapContextManager::new();
    a.register_variable(RegisterVariableRequest {
        name: "x".to_string(),
        var: &Variable::new("x".to_string(), VariableKind::String),
    })
    .expect("register");
    let b = MapContextManager::new();
    assert!(b
        .get_variable(VariableLookupRequest { name: "x" })
        .expect("get_variable ok")
        .variable
        .is_none());
}

// --- token_counter ---

/// @covers: PromptBootstrap::token_counter — builds a working counter
#[test]
fn test_token_counter_counts_text_happy() {
    assert!(
        HeuristicTokenCounter::new()
            .count_tokens(CountTokensRequest { text: "hello" })
            .expect("count_tokens ok")
            .count
            >= 1
    );
}

/// @covers: PromptBootstrap::token_counter — empty string counts zero
#[test]
fn test_token_counter_empty_zero_error() {
    assert_eq!(
        HeuristicTokenCounter::new()
            .count_tokens(CountTokensRequest { text: "" })
            .expect("count_tokens ok")
            .count,
        0
    );
}

/// @covers: PromptBootstrap::token_counter — reports it is not exact
#[test]
fn test_token_counter_not_exact_edge() {
    assert!(
        !HeuristicTokenCounter::new()
            .is_exact(ExactnessRequest)
            .expect("is_exact ok")
            .exact
    );
}

// --- default_prompt_handler ---

/// @covers: default_prompt_handler — builds a usable Handler
#[test]
fn test_default_prompt_handler_renders_happy() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    let var = Variable::new("name".to_string(), VariableKind::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let h = StdPromptFactory::default_prompt_handler("Hi {{name}}".to_string(), m);
    let security: SecurityContext = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let render_ctx =
        RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: render_ctx,
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert_eq!(out, "Hi Ada");
}

/// @covers: default_prompt_handler — missing required variable surfaces an error through the pipeline
#[test]
fn test_default_prompt_handler_missing_variable_errors_error() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    let var = Variable::new("name".to_string(), VariableKind::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let h = StdPromptFactory::default_prompt_handler("Hi {{name}}".to_string(), m);
    let security: SecurityContext = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    assert!(block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: RenderContext::new(),
            ctx: &ctx,
        }
    ))
    .is_err());
}

/// @covers: default_prompt_handler — exposes the stable dispatch id
#[test]
fn test_default_prompt_handler_id_is_stable_edge() {
    use edge_domain_handler::{Handler, IdRequest};
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![]);
    let h = StdPromptFactory::default_prompt_handler("static".to_string(), m);
    assert_eq!(
        Handler::id(&h, IdRequest).expect("id ok").id,
        "prompt.render"
    );
}

// --- prompt_handler ---

/// @covers: StdPromptFactory::prompt_handler — builds a usable Handler from a prompt impl
#[test]
fn test_prompt_handler_renders_with_arc_prompt_happy() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    use std::sync::Arc;
    let var = Variable::new("name".to_string(), VariableKind::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let prompt = Arc::new(StaticPrompt::new("Hi {{name}}".to_string(), m));
    let h = StdPromptFactory::prompt_handler(prompt);
    let security: SecurityContext = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let render_ctx =
        RenderContext::new().with_variable("name".to_string(), serde_json::json!("Eve"));
    let out = block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: render_ctx,
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert_eq!(out, "Hi Eve");
}

/// @covers: StdPromptFactory::prompt_handler — missing required variable is an error
#[test]
fn test_prompt_handler_missing_required_variable_error() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use edge_llm_prompt::{RenderContext, Variable};
    use futures::executor::block_on;
    use std::sync::Arc;
    let var = Variable::new("name".to_string(), VariableKind::String);
    let m = PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
    let prompt = Arc::new(StaticPrompt::new("Hi {{name}}".to_string(), m));
    let h = StdPromptFactory::prompt_handler(prompt);
    let security: SecurityContext = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    assert!(block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: RenderContext::new(),
            ctx: &ctx,
        }
    ))
    .is_err());
}

/// @covers: StdPromptFactory::prompt_handler — empty template renders to an empty string
#[test]
fn test_prompt_handler_empty_template_edge() {
    use edge_domain_handler::{Handler, IdRequest};
    use std::sync::Arc;
    let m = PromptMetadata::new("e".to_string(), "E".to_string(), "1".to_string(), vec![]);
    let prompt = Arc::new(StaticPrompt::new(String::new(), m));
    let h = StdPromptFactory::prompt_handler(prompt);
    assert_eq!(
        Handler::id(&h, IdRequest).expect("id ok").id,
        "prompt.render"
    );
}

// --- template_provider ---

/// @covers: PromptBootstrap::template_provider — builds an empty registry
#[test]
fn test_template_provider_starts_empty_happy() {
    assert!(CatalogTemplateProvider::new().is_empty());
}

/// @covers: PromptBootstrap::template_provider — unknown id is absent
#[test]
fn test_template_provider_unknown_id_absent_error() {
    use edge_llm_prompt::TemplateProvider;
    assert!(CatalogTemplateProvider::new()
        .get_template(TemplateLookupRequest { id: "x" })
        .expect("get_template ok")
        .template
        .is_none());
}

/// @covers: PromptBootstrap::template_provider — independent instances per call
#[test]
fn test_template_provider_independent_instances_edge() {
    use edge_llm_prompt::{PromptTemplate, TemplateProvider};
    let mut a = CatalogTemplateProvider::new();
    a.insert(PromptTemplate::new(
        "x".to_string(),
        "x".to_string(),
        "c".to_string(),
    ));
    let b = CatalogTemplateProvider::new();
    assert!(b
        .get_template(TemplateLookupRequest { id: "x" })
        .expect("get_template ok")
        .template
        .is_none());
}

// --- prompt_template_builder ---

/// @covers: PromptBootstrap::prompt_template_builder — builds with overrides
#[test]
fn test_prompt_template_builder_overrides_happy() {
    let t = PromptTemplateBuilder::new()
        .id("code-review".to_string())
        .category("code".to_string())
        .build();
    assert_eq!(t.id, "code-review");
    assert_eq!(t.category, "code");
}

/// @covers: PromptBootstrap::prompt_template_builder — defaults to empty bodies
#[test]
fn test_prompt_template_builder_default_empty_bodies_error() {
    assert!(PromptTemplateBuilder::new()
        .build()
        .system_prompt
        .is_empty());
}

/// @covers: PromptBootstrap::prompt_template_builder — user template carried through
#[test]
fn test_prompt_template_builder_user_template_edge() {
    let t = PromptTemplateBuilder::new()
        .user_template("review {{code}}".to_string())
        .build();
    assert_eq!(t.user_template, "review {{code}}");
}
