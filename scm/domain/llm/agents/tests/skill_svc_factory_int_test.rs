//! SAF service tests — skill factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::SKILL_SVC_FACTORY;

/// @covers: SKILL_SVC_FACTORY — equals the canonical string
#[test]
fn test_skill_svc_factory_equals_canonical_happy() {
    assert_eq!(SKILL_SVC_FACTORY, "skill_svc_factory");
}

/// @covers: SKILL_SVC_FACTORY — is non-empty
#[test]
fn test_skill_svc_factory_is_non_empty_error() {
    assert!(!SKILL_SVC_FACTORY.is_empty());
}

/// @covers: SKILL_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_skill_svc_factory_is_valid_identifier_edge() {
    assert!(SKILL_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
