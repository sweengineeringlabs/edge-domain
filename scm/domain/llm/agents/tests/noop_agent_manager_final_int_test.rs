#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `AgentManager` constructed via `saf::noop_agent_manager`.

use edge_llm_agent::{
    AgentLoadRequest, AgentLookupRequest, AgentManager, ListAgentIdsRequest, NoopAgentManager,
};

#[test]
fn test_noop_agent_manager_lists_no_agents() {
    let ids = NoopAgentManager
        .list_agent_ids(ListAgentIdsRequest)
        .map(|r| r.ids)
        .unwrap_or_default();
    assert!(ids.is_empty());
}

#[test]
fn test_noop_agent_manager_agent_lookup_fails() {
    assert!(NoopAgentManager
        .agent(AgentLookupRequest { id: "missing" })
        .is_err());
}

#[test]
fn test_noop_agent_manager_load_agent_fails() {
    let result =
        futures::executor::block_on(NoopAgentManager.load_agent(AgentLoadRequest { spec: "spec" }));
    assert!(result.is_err());
}
