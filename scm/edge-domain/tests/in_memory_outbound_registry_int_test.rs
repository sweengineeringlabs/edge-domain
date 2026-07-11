//! Integration tests for `InMemoryOutboundRegistry` construction.
// @allow: no_mocks_in_integration — InMemoryOutboundRegistry is the production-shipped reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{InMemoryOutboundRegistry, OutboundIsEmptyRequest, OutboundRegistry};

/// @covers: InMemoryOutboundRegistry::new
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg: InMemoryOutboundRegistry<String> = InMemoryOutboundRegistry::new();
    assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
}

/// @covers: InMemoryOutboundRegistry::default
#[test]
fn test_default_matches_new_error() {
    let reg: InMemoryOutboundRegistry<String> = InMemoryOutboundRegistry::default();
    assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
}

/// @covers: InMemoryOutboundRegistry::new
#[test]
fn test_new_distinct_instances_are_independent_edge() {
    let a: InMemoryOutboundRegistry<u32> = InMemoryOutboundRegistry::new();
    let b: InMemoryOutboundRegistry<u32> = InMemoryOutboundRegistry::new();
    assert!(!std::ptr::eq(&a, &b));
}
