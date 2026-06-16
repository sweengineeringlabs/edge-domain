//! Integration tests for `InMemoryRegistry` — covers the types/ file directly.
// @allow: no_mocks_in_integration — InMemoryRegistry is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_registry::{InMemoryRegistry, Registry};

/// @covers: InMemoryRegistry::new — starts empty
#[test]
fn test_new_starts_empty_happy() {
    let r: InMemoryRegistry<str> = InMemoryRegistry::new();
    assert!(r.is_empty());
}

/// @covers: InMemoryRegistry (Default) — equivalent to new, starts empty
#[test]
fn test_default_starts_empty_error() {
    let r: InMemoryRegistry<str> = InMemoryRegistry::default();
    assert_eq!(r.len(), 0);
}

/// @covers: InMemoryRegistry — round-trips an unsized (`str`) entry
#[test]
fn test_round_trip_unsized_entry_edge() {
    let r: InMemoryRegistry<str> = InMemoryRegistry::new();
    r.register("k", Arc::from("v"));
    assert_eq!(r.get("k").as_deref(), Some("v"));
}
