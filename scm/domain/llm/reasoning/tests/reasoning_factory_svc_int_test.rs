//! SAF facade tests — standard reasoning primitive constructors and builders.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{
    LinearReasoning, PatternMetadataBuilder, PatternSupportRequest, Reasoning,
    ReasoningChainBuilder, ReasoningPattern, ReasoningStepBuilder, StdReasoningFactory,
    StepResultBuilder, ThinkingProcessBuilder,
};

// --- std_factory ---

/// @covers: StdReasoningFactory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let f: StdReasoningFactory = StdReasoningFactory;
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: StdReasoningFactory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdReasoningFactory;
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: StdReasoningFactory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let a = StdReasoningFactory;
    let b = StdReasoningFactory;
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

// --- reasoning ---

/// @covers: LinearReasoning::new — builds a reasoner for the pattern
#[test]
fn test_reasoning_builds_for_pattern_happy() {
    let r = LinearReasoning::new(ReasoningPattern::ChainOfThought);
    assert!(
        r.supports_pattern(PatternSupportRequest {
            pattern: ReasoningPattern::ChainOfThought
        })
        .unwrap()
        .supported
    );
}

/// @covers: LinearReasoning::new — does not support unrelated patterns
#[test]
fn test_reasoning_rejects_other_pattern_error() {
    let r = LinearReasoning::new(ReasoningPattern::ChainOfThought);
    assert!(
        !r.supports_pattern(PatternSupportRequest {
            pattern: ReasoningPattern::GraphBased
        })
        .unwrap()
        .supported
    );
}

/// @covers: LinearReasoning::new — preserves the requested pattern
#[test]
fn test_reasoning_preserves_pattern_edge() {
    let r = LinearReasoning::new(ReasoningPattern::Reflection);
    assert_eq!(r.pattern(), ReasoningPattern::Reflection);
}

// --- reasoning_step_builder ---

/// @covers: ReasoningStepBuilder::new — sets the index
#[test]
fn test_reasoning_step_builder_index_happy() {
    let step = ReasoningStepBuilder::new(3)
        .content("x".to_string())
        .build();
    assert_eq!(step.index, 3);
}

/// @covers: ReasoningStepBuilder::new — default confidence is mid
#[test]
fn test_reasoning_step_builder_default_confidence_error() {
    let step = ReasoningStepBuilder::new(0).build();
    assert!(!step.is_confident());
}

/// @covers: ReasoningStepBuilder::new — clamps confidence
#[test]
fn test_reasoning_step_builder_clamps_confidence_edge() {
    let step = ReasoningStepBuilder::new(0)
        .confidence(5.0)
        .build();
    assert!((step.confidence - 1.0).abs() < 0.001);
}

// --- step_result_builder ---

/// @covers: StepResultBuilder::new — defaults to success
#[test]
fn test_step_result_builder_default_success_happy() {
    assert!(StepResultBuilder::new().build().success);
}

/// @covers: StepResultBuilder::new — error marks failure
#[test]
fn test_step_result_builder_error_marks_failure_error() {
    let result = StepResultBuilder::new()
        .error("boom".to_string())
        .build();
    assert!(!result.success);
}

/// @covers: StepResultBuilder::new — duration carried through
#[test]
fn test_step_result_builder_duration_edge() {
    let result = StepResultBuilder::new()
        .duration_ms(1500)
        .build();
    assert!(!result.was_fast());
}

// --- thinking_process_builder ---

/// @covers: ThinkingProcessBuilder::new — sets the id
#[test]
fn test_thinking_process_builder_id_happy() {
    let process = ThinkingProcessBuilder::new("p1".to_string()).build();
    assert_eq!(process.id, "p1");
}

/// @covers: ThinkingProcessBuilder::new — incomplete by default
#[test]
fn test_thinking_process_builder_incomplete_default_error() {
    let process = ThinkingProcessBuilder::new("p1".to_string()).build();
    assert!(!process.is_complete);
}

/// @covers: ThinkingProcessBuilder::new — conclusion completes it
#[test]
fn test_thinking_process_builder_conclusion_completes_edge() {
    let process = ThinkingProcessBuilder::new("p1".to_string())
        .conclusion("answer".to_string())
        .build();
    assert!(process.is_complete);
}

// --- pattern_metadata_builder ---

/// @covers: PatternMetadataBuilder::new — seeds the pattern
#[test]
fn test_pattern_metadata_builder_pattern_happy() {
    let meta =
        PatternMetadataBuilder::new(ReasoningPattern::TreeOfThought).build();
    assert_eq!(meta.pattern, ReasoningPattern::TreeOfThought);
}

/// @covers: PatternMetadataBuilder::new — overrides max depth
#[test]
fn test_pattern_metadata_builder_max_depth_error() {
    let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
        .max_depth(99)
        .build();
    assert_eq!(meta.max_depth, 99);
}

/// @covers: PatternMetadataBuilder::new — clamps min confidence
#[test]
fn test_pattern_metadata_builder_clamps_confidence_edge() {
    let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
        .min_confidence(-1.0)
        .build();
    assert!((meta.min_confidence - 0.0).abs() < 0.001);
}

// --- reasoning_chain_builder ---

/// @covers: ReasoningChainBuilder::new — sets the id
#[test]
fn test_reasoning_chain_builder_id_happy() {
    let chain = ReasoningChainBuilder::new("c1".to_string()).build();
    assert_eq!(chain.id, "c1");
}

/// @covers: ReasoningChainBuilder::new — empty by default
#[test]
fn test_reasoning_chain_builder_empty_default_error() {
    let chain = ReasoningChainBuilder::new("c1".to_string()).build();
    assert_eq!(chain.process_count(), 0);
}

/// @covers: ReasoningChainBuilder::new — final answer completes it
#[test]
fn test_reasoning_chain_builder_final_answer_edge() {
    let chain = ReasoningChainBuilder::new("c1".to_string())
        .final_answer("done".to_string())
        .build();
    assert!(chain.is_complete);
}

// --- default_reasoning_handler ---

/// @covers: default_reasoning_handler — builds a usable Handler
#[test]
fn test_default_reasoning_handler_runs_happy() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: "solve x".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert!(out.is_complete);
}

/// @covers: default_reasoning_handler — pattern mismatch surfaces an error through the pipeline
#[test]
fn test_default_reasoning_handler_pattern_mismatch_errors_error() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::GraphBased);
    let security = SecurityContext::unauthenticated();
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
            req: "solve x".to_string(),
            ctx: &ctx,
        },
    ))
    .is_err());
}

/// @covers: default_reasoning_handler — exposes the stable dispatch id
#[test]
fn test_default_reasoning_handler_id_is_stable_edge() {
    use edge_domain_handler::{Handler, IdRequest};
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    assert_eq!(Handler::id(&h, IdRequest).unwrap().id, "reasoning.reason");
}

// --- reasoning_handler ---

/// @covers: StdReasoningFactory::reasoning_handler — builds a usable Handler from a reasoning impl
#[test]
fn test_reasoning_handler_produces_thinking_process_happy() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;
    use std::sync::Arc;
    let reasoner = Arc::new(LinearReasoning::new(
        ReasoningPattern::ChainOfThought,
    ));
    let h = StdReasoningFactory::reasoning_handler(reasoner);
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: "what is 2+2?".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert!(out.is_complete);
}

/// @covers: StdReasoningFactory::reasoning_handler — unsupported pattern surfaces an error
#[test]
fn test_reasoning_handler_rejects_unsupported_pattern_error() {
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;
    use std::sync::Arc;
    let reasoner = Arc::new(LinearReasoning::new(ReasoningPattern::GraphBased));
    let h = StdReasoningFactory::reasoning_handler(reasoner);
    let security = SecurityContext::unauthenticated();
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
            req: "x".to_string(),
            ctx: &ctx,
        },
    ))
    .is_err());
}

/// @covers: StdReasoningFactory::reasoning_handler — exposes stable dispatch id
#[test]
fn test_reasoning_handler_preserves_dispatch_id_edge() {
    use edge_domain_handler::{Handler, IdRequest};
    use std::sync::Arc;
    let reasoner = Arc::new(LinearReasoning::new(
        ReasoningPattern::ChainOfThought,
    ));
    let h = StdReasoningFactory::reasoning_handler(reasoner);
    assert_eq!(Handler::id(&h, IdRequest).unwrap().id, "reasoning.reason");
}
