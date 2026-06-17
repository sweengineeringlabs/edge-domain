//! Tests for the `ReasoningChainBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{ReasoningChainBuilder, ThinkingProcess};

/// @covers: ReasoningChainBuilder — builds with appended processes
#[test]
fn test_reasoning_chain_builder_builds_with_processes() {
    let process = ThinkingProcess::new("p".to_string(), "solve".to_string());
    let chain = ReasoningChainBuilder::new("c1".to_string())
        .process(process)
        .build();
    assert_eq!(chain.process_count(), 1);
}

/// @covers: ReasoningChainBuilder — empty by default
#[test]
fn test_reasoning_chain_builder_empty_default() {
    assert_eq!(
        ReasoningChainBuilder::new("c1".to_string())
            .build()
            .process_count(),
        0
    );
}

/// @covers: ReasoningChainBuilder — final answer completes the chain
#[test]
fn test_reasoning_chain_builder_final_answer_completes() {
    let chain = ReasoningChainBuilder::new("c1".to_string())
        .final_answer("done".to_string())
        .build();
    assert!(chain.is_complete);
    assert_eq!(chain.final_answer.as_deref(), Some("done"));
}
