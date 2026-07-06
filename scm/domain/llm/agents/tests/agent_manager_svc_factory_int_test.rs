//! SAF service tests — agent-manager factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::AGENT_MANAGER_SVC_FACTORY;

/// @covers: AGENT_MANAGER_SVC_FACTORY — equals the canonical string
#[test]
fn test_agent_manager_svc_factory_equals_canonical_happy() {
    assert_eq!(AGENT_MANAGER_SVC_FACTORY, "agent_manager_svc_factory");
}

/// @covers: AGENT_MANAGER_SVC_FACTORY — is non-empty
#[test]
fn test_agent_manager_svc_factory_is_non_empty_error() {
    assert!(!AGENT_MANAGER_SVC_FACTORY.is_empty());
}

/// @covers: AGENT_MANAGER_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_agent_manager_svc_factory_is_valid_identifier_edge() {
    assert!(AGENT_MANAGER_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
