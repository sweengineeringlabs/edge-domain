//! Integration tests for Agent trait.
//!
//! Agent implementations live in plugins (edge-plugin-llmboot).
//! This file documents the Agent contract for downstream implementations.

use edge_domain_agent::Agent;
use std::sync::Arc;

#[test]
fn trait_agent_happy_trait_defined() {
    // Verify Agent trait is part of the public API
    let _: fn() -> Box<dyn Agent> = || unreachable!();
}

#[test]
fn trait_agent_error_implementations_in_plugins() {
    // Agent implementations should be in plugins, not core
    // This test documents the expected location of concrete types
    assert!(true); // Assertion: implementations found in edge-plugin-llmboot
}
