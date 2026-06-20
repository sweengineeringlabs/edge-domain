//! Tests for the `StdReasoningFactory` concrete `ReasoningBootstrap` implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{Reasoning, ReasoningBootstrap, ReasoningPattern, StdReasoningFactory};

/// @covers: StdReasoningFactory — std_factory returns the factory instance
#[test]
fn test_std_reasoning_factory_std_factory_returns_instance() {
    let _factory: StdReasoningFactory = StdReasoningFactory::std_factory();
}

/// @covers: StdReasoningFactory — is zero-sized
#[test]
fn test_std_reasoning_factory_is_zero_sized() {
    assert_eq!(std::mem::size_of::<StdReasoningFactory>(), 0);
}

/// @covers: StdReasoningFactory — builds a reasoner via the factory
#[test]
fn test_std_reasoning_factory_builds_reasoner() {
    let r = StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought);
    assert!(r.supports_pattern(ReasoningPattern::ChainOfThought));
}
