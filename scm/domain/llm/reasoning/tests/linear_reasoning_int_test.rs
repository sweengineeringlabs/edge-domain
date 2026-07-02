//! Tests for the `LinearReasoning` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{LinearReasoning, ReasonRequest, Reasoning, ReasoningPattern};
use futures::executor::block_on;

/// @covers: LinearReasoning::new — reports the configured pattern
#[test]
fn test_linear_reasoning_reports_pattern() {
    let r = LinearReasoning::new(ReasoningPattern::ChainOfThought);
    assert_eq!(r.pattern(), ReasoningPattern::ChainOfThought);
}

/// @covers: LinearReasoning — reasons to a completed process
#[test]
fn test_linear_reasoning_completes_process() {
    let r = LinearReasoning::new(ReasoningPattern::ChainOfThought);
    let resp = block_on(r.reason(ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::ChainOfThought,
    }))
    .expect("reasoning should succeed");
    assert!(resp.process.is_complete);
}

/// @covers: LinearReasoning — clone preserves the pattern
#[test]
fn test_linear_reasoning_clone_preserves_pattern() {
    let r = LinearReasoning::new(ReasoningPattern::Reflection);
    assert_eq!(r.clone().pattern(), ReasoningPattern::Reflection);
}
