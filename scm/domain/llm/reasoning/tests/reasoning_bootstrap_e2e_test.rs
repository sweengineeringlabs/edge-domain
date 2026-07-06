//! Layer-level e2e coverage for the `ReasoningBootstrap` trait via a test-double implementer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::*;

struct BootstrapDouble;

impl ReasoningBootstrap for BootstrapDouble {}

/// @covers: ReasoningBootstrap::bootstrap_name — default impl reports "reasoning"
#[test]
fn test_bootstrap_name_default_reports_reasoning_happy() {
    let name = BootstrapDouble
        .bootstrap_name(ReasoningBootstrapNameRequest)
        .expect("bootstrap_name ok")
        .name;
    assert_eq!(name, "reasoning");
}

/// @covers: ReasoningBootstrap::reasoning — constructs a reasoner bound to the given pattern
#[test]
fn test_reasoning_constructs_bound_reasoner_happy() {
    let reasoner = BootstrapDouble::reasoning(ReasoningPattern::Reflection);
    assert_eq!(reasoner.pattern(), ReasoningPattern::Reflection);
}

/// @covers: ReasoningBootstrap::pattern_metadata_builder — edge case with default pattern
#[test]
fn test_pattern_metadata_builder_default_pattern_edge() {
    let metadata =
        BootstrapDouble::pattern_metadata_builder(ReasoningPattern::ChainOfThought).build();
    assert_eq!(metadata.pattern, ReasoningPattern::ChainOfThought);
}

/// @covers: ReasoningBootstrap::reasoning_step_builder — builds a step at the given index
#[test]
fn test_reasoning_step_builder_builds_at_index_error() {
    let step = BootstrapDouble::reasoning_step_builder(2).build();
    assert_eq!(step.index, 2);
}
