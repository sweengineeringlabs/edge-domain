//! SAF facade tests — `ReasoningBootstrap` constructors and builders.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{Reasoning, ReasoningBootstrap, ReasoningPattern, StdReasoningFactory};

// --- std_factory ---

/// @covers: ReasoningBootstrap::std_factory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let _f: StdReasoningFactory = StdReasoningFactory::std_factory();
}

/// @covers: ReasoningBootstrap::std_factory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdReasoningFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: ReasoningBootstrap::std_factory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let _a = StdReasoningFactory::std_factory();
    let _b = StdReasoningFactory::std_factory();
}

// --- reasoning ---

/// @covers: ReasoningBootstrap::reasoning — builds a reasoner for the pattern
#[test]
fn test_reasoning_builds_for_pattern_happy() {
    let r = StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought);
    assert!(r.supports_pattern(ReasoningPattern::ChainOfThought));
}

/// @covers: ReasoningBootstrap::reasoning — does not support unrelated patterns
#[test]
fn test_reasoning_rejects_other_pattern_error() {
    let r = StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought);
    assert!(!r.supports_pattern(ReasoningPattern::GraphBased));
}

/// @covers: ReasoningBootstrap::reasoning — preserves the requested pattern
#[test]
fn test_reasoning_preserves_pattern_edge() {
    let r = StdReasoningFactory::reasoning(ReasoningPattern::Reflection);
    assert_eq!(r.pattern(), ReasoningPattern::Reflection);
}

// --- reasoning_step_builder ---

/// @covers: ReasoningBootstrap::reasoning_step_builder — sets the index
#[test]
fn test_reasoning_step_builder_index_happy() {
    let step = StdReasoningFactory::reasoning_step_builder(3)
        .content("x".to_string())
        .build();
    assert_eq!(step.index, 3);
}

/// @covers: ReasoningBootstrap::reasoning_step_builder — default confidence is mid
#[test]
fn test_reasoning_step_builder_default_confidence_error() {
    let step = StdReasoningFactory::reasoning_step_builder(0).build();
    assert!(!step.is_confident());
}

/// @covers: ReasoningBootstrap::reasoning_step_builder — clamps confidence
#[test]
fn test_reasoning_step_builder_clamps_confidence_edge() {
    let step = StdReasoningFactory::reasoning_step_builder(0)
        .confidence(5.0)
        .build();
    assert!((step.confidence - 1.0).abs() < 0.001);
}

// --- step_result_builder ---

/// @covers: ReasoningBootstrap::step_result_builder — defaults to success
#[test]
fn test_step_result_builder_default_success_happy() {
    assert!(StdReasoningFactory::step_result_builder().build().success);
}

/// @covers: ReasoningBootstrap::step_result_builder — error marks failure
#[test]
fn test_step_result_builder_error_marks_failure_error() {
    let result = StdReasoningFactory::step_result_builder()
        .error("boom".to_string())
        .build();
    assert!(!result.success);
}

/// @covers: ReasoningBootstrap::step_result_builder — duration carried through
#[test]
fn test_step_result_builder_duration_edge() {
    let result = StdReasoningFactory::step_result_builder()
        .duration_ms(1500)
        .build();
    assert!(!result.was_fast());
}

// --- thinking_process_builder ---

/// @covers: ReasoningBootstrap::thinking_process_builder — sets the id
#[test]
fn test_thinking_process_builder_id_happy() {
    let process = StdReasoningFactory::thinking_process_builder("p1".to_string()).build();
    assert_eq!(process.id, "p1");
}

/// @covers: ReasoningBootstrap::thinking_process_builder — incomplete by default
#[test]
fn test_thinking_process_builder_incomplete_default_error() {
    let process = StdReasoningFactory::thinking_process_builder("p1".to_string()).build();
    assert!(!process.is_complete);
}

/// @covers: ReasoningBootstrap::thinking_process_builder — conclusion completes it
#[test]
fn test_thinking_process_builder_conclusion_completes_edge() {
    let process = StdReasoningFactory::thinking_process_builder("p1".to_string())
        .conclusion("answer".to_string())
        .build();
    assert!(process.is_complete);
}

// --- pattern_metadata_builder ---

/// @covers: ReasoningBootstrap::pattern_metadata_builder — seeds the pattern
#[test]
fn test_pattern_metadata_builder_pattern_happy() {
    let meta =
        StdReasoningFactory::pattern_metadata_builder(ReasoningPattern::TreeOfThought).build();
    assert_eq!(meta.pattern, ReasoningPattern::TreeOfThought);
}

/// @covers: ReasoningBootstrap::pattern_metadata_builder — overrides max depth
#[test]
fn test_pattern_metadata_builder_max_depth_error() {
    let meta = StdReasoningFactory::pattern_metadata_builder(ReasoningPattern::ChainOfThought)
        .max_depth(99)
        .build();
    assert_eq!(meta.max_depth, 99);
}

/// @covers: ReasoningBootstrap::pattern_metadata_builder — clamps min confidence
#[test]
fn test_pattern_metadata_builder_clamps_confidence_edge() {
    let meta = StdReasoningFactory::pattern_metadata_builder(ReasoningPattern::ChainOfThought)
        .min_confidence(-1.0)
        .build();
    assert!((meta.min_confidence - 0.0).abs() < 0.001);
}

// --- reasoning_chain_builder ---

/// @covers: ReasoningBootstrap::reasoning_chain_builder — sets the id
#[test]
fn test_reasoning_chain_builder_id_happy() {
    let chain = StdReasoningFactory::reasoning_chain_builder("c1".to_string()).build();
    assert_eq!(chain.id, "c1");
}

/// @covers: ReasoningBootstrap::reasoning_chain_builder — empty by default
#[test]
fn test_reasoning_chain_builder_empty_default_error() {
    let chain = StdReasoningFactory::reasoning_chain_builder("c1".to_string()).build();
    assert_eq!(chain.process_count(), 0);
}

/// @covers: ReasoningBootstrap::reasoning_chain_builder — final answer completes it
#[test]
fn test_reasoning_chain_builder_final_answer_edge() {
    let chain = StdReasoningFactory::reasoning_chain_builder("c1".to_string())
        .final_answer("done".to_string())
        .build();
    assert!(chain.is_complete);
}

// --- default_reasoning_handler ---

/// @covers: default_reasoning_handler — builds a usable Handler
#[test]
fn test_default_reasoning_handler_runs_happy() {
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let out = block_on(Handler::execute(&h, "solve x".to_string(), ctx)).expect("ok");
    assert!(out.is_complete);
}

/// @covers: default_reasoning_handler — pattern mismatch surfaces an error through the pipeline
#[test]
fn test_default_reasoning_handler_pattern_mismatch_errors_error() {
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::GraphBased);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    assert!(block_on(Handler::execute(&h, "solve x".to_string(), ctx)).is_err());
}

/// @covers: default_reasoning_handler — exposes the stable dispatch id
#[test]
fn test_default_reasoning_handler_id_is_stable_edge() {
    use edge_domain_handler::Handler;
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    assert_eq!(Handler::id(&h), "reasoning.reason");
}

// --- reasoning_handler ---

/// @covers: StdReasoningFactory::reasoning_handler — builds a usable Handler from a reasoning impl
#[test]
fn test_reasoning_handler_produces_thinking_process_happy() {
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;
    use std::sync::Arc;
    let reasoner = Arc::new(StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought));
    let h = StdReasoningFactory::reasoning_handler(reasoner);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let out = block_on(Handler::execute(&h, "what is 2+2?".to_string(), ctx)).expect("ok");
    assert!(out.is_complete);
}

/// @covers: StdReasoningFactory::reasoning_handler — unsupported pattern surfaces an error
#[test]
fn test_reasoning_handler_rejects_unsupported_pattern_error() {
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;
    use std::sync::Arc;
    let reasoner = Arc::new(StdReasoningFactory::reasoning(ReasoningPattern::GraphBased));
    let h = StdReasoningFactory::reasoning_handler(reasoner);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    assert!(block_on(Handler::execute(&h, "x".to_string(), ctx)).is_err());
}

/// @covers: StdReasoningFactory::reasoning_handler — exposes stable dispatch id
#[test]
fn test_reasoning_handler_preserves_dispatch_id_edge() {
    use edge_domain_handler::Handler;
    use std::sync::Arc;
    let reasoner = Arc::new(StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought));
    let h = StdReasoningFactory::reasoning_handler(reasoner);
    assert_eq!(Handler::id(&h), "reasoning.reason");
}
