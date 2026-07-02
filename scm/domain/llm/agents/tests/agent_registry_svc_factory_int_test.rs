//! SAF service tests — agent-registry factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::AGENT_REGISTRY_SVC_FACTORY;

/// @covers: AGENT_REGISTRY_SVC_FACTORY — equals the canonical string
#[test]
fn test_agent_registry_svc_factory_equals_canonical_happy() {
    assert_eq!(AGENT_REGISTRY_SVC_FACTORY, "agent_registry_svc_factory");
}

/// @covers: AGENT_REGISTRY_SVC_FACTORY — is non-empty
#[test]
fn test_agent_registry_svc_factory_is_non_empty_error() {
    assert!(!AGENT_REGISTRY_SVC_FACTORY.is_empty());
}

/// @covers: AGENT_REGISTRY_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_agent_registry_svc_factory_is_valid_identifier_edge() {
    assert!(AGENT_REGISTRY_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
