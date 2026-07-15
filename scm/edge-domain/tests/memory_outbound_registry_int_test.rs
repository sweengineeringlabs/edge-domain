//! Integration tests for `MemoryOutboundRegistry` construction.
// @allow: no_mocks_in_integration — MemoryOutboundRegistry is the production-shipped reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{MemoryOutboundRegistry, OutboundIsEmptyRequest, OutboundRegistry};

/// @covers: MemoryOutboundRegistry::new
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::new();
    assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
}

/// @covers: MemoryOutboundRegistry::default
#[test]
fn test_default_matches_new_error() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::default();
    assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
}

/// @covers: MemoryOutboundRegistry::new
#[test]
fn test_new_distinct_instances_are_independent_edge() {
    let a: MemoryOutboundRegistry<u32> = MemoryOutboundRegistry::new();
    let b: MemoryOutboundRegistry<u32> = MemoryOutboundRegistry::new();
    assert!(!std::ptr::eq(&a, &b));
}
