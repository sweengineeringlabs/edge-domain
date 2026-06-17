//! Tests for the `ReasoningChain` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{ReasoningChain, ThinkingProcess};

fn process(complete: bool) -> ThinkingProcess {
    let p = ThinkingProcess::new("p".to_string(), "solve".to_string());
    if complete {
        p.complete("answer".to_string())
    } else {
        p
    }
}

/// @covers: ReasoningChain::add_process — counts processes
#[test]
fn test_add_process_counts() {
    let mut chain = ReasoningChain::new("c".to_string());
    chain.add_process(process(true));
    assert_eq!(chain.process_count(), 1);
}

/// @covers: ReasoningChain::all_complete — false with an incomplete process
#[test]
fn test_all_complete_false_with_incomplete() {
    let mut chain = ReasoningChain::new("c".to_string());
    chain.add_process(process(false));
    assert!(!chain.all_complete());
}

/// @covers: ReasoningChain::all_complete — true when all complete
#[test]
fn test_all_complete_true_when_all_done() {
    let mut chain = ReasoningChain::new("c".to_string());
    chain.add_process(process(true));
    assert!(chain.all_complete());
}

/// @covers: ReasoningChain::complete — sets the final answer
#[test]
fn test_complete_sets_final_answer() {
    let chain = ReasoningChain::new("c".to_string()).complete("final".to_string());
    assert_eq!(chain.final_answer.as_deref(), Some("final"));
}

/// @covers: ReasoningChain::average_confidence — empty chain is zero
#[test]
fn test_average_confidence_empty_is_zero() {
    let chain = ReasoningChain::new("c".to_string());
    assert!((chain.average_confidence() - 0.0).abs() < 0.001);
}

/// @covers: ReasoningChain — serde round-trip
#[test]
fn test_reasoning_chain_serde_roundtrip() {
    let chain = ReasoningChain::new("c".to_string());
    let json = serde_json::to_string(&chain).expect("serialize");
    let back: ReasoningChain = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.id, "c");
}
