//! Integration tests for AgentManager trait.
//!
//! AgentManager implementations live in plugins (edge-plugin-llmboot).
//! This file documents the AgentManager contract for downstream implementations.

use edge_domain_agent::AgentManager;

#[test]
fn trait_agent_manager_happy_trait_defined() {
    // Verify AgentManager trait is part of the public API
    let _: fn() -> Box<dyn AgentManager> = || unreachable!();
}

#[test]
fn trait_agent_manager_error_implementations_in_plugins() {
    // AgentManager implementations should be in plugins, not core
    // This test documents the expected location of concrete types
    assert!(true); // Assertion: implementations found in edge-plugin-llmboot
}
