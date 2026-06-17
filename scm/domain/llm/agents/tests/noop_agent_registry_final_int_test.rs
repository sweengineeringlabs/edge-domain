#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `AgentRegistry` constructed via `saf::noop_agent_registry`.

use edge_domain_registry::Registry;
use edge_llm_agent::{AgentRegistry, NoopAgentRegistry};

#[test]
fn test_noop_agent_registry_is_empty() {
    assert_eq!(NoopAgentRegistry.list_ids().len(), 0);
}

#[test]
fn test_noop_agent_registry_get_returns_none() {
    assert!(NoopAgentRegistry.get("missing").is_none());
}

#[test]
fn test_noop_agent_registry_metadata_reports_missing() {
    assert!(NoopAgentRegistry.metadata("missing").is_err());
}
