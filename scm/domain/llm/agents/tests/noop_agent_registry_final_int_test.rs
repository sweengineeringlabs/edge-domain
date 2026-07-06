#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `AgentRegistry` constructed via `saf::noop_agent_registry`.

use edge_domain_registry::{ListIdsRequest, Registry, RegistryLookupRequest};
use edge_llm_agent::{AgentMetadataLookupRequest, AgentRegistry, NoopAgentRegistry};

#[test]
fn test_noop_agent_registry_is_empty() {
    assert_eq!(
        NoopAgentRegistry.list_ids(ListIdsRequest).unwrap().ids.len(),
        0
    );
}

#[test]
fn test_noop_agent_registry_get_returns_none() {
    assert!(NoopAgentRegistry
        .get(RegistryLookupRequest {
            id: "missing".to_string()
        })
        .unwrap()
        .entry
        .is_none());
}

#[test]
fn test_noop_agent_registry_metadata_reports_missing() {
    assert!(NoopAgentRegistry
        .metadata(AgentMetadataLookupRequest { id: "missing" })
        .is_err());
}
