//! Integration tests for `InMemoryRegistry` — covers the types/ file directly.
// @allow: no_mocks_in_integration — InMemoryRegistry is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_registry::{
    EmptinessRequest, InMemoryRegistry, LenRequest, RegisterRequest, Registry,
    RegistryLookupRequest,
};

/// @covers: InMemoryRegistry::new — starts empty
#[test]
fn test_new_starts_empty_happy() {
    let r: InMemoryRegistry<str> = InMemoryRegistry::new();
    assert!(r.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: InMemoryRegistry (Default) — equivalent to new, starts empty
#[test]
fn test_default_starts_empty_error() {
    let r: InMemoryRegistry<str> = InMemoryRegistry::default();
    assert_eq!(r.len(LenRequest).unwrap().count, 0);
}

/// @covers: InMemoryRegistry — round-trips an unsized (`str`) entry
#[test]
fn test_round_trip_unsized_entry_edge() {
    let r: InMemoryRegistry<str> = InMemoryRegistry::new();
    r.register(RegisterRequest {
        id: "k".to_string(),
        entry: Arc::from("v"),
    })
    .unwrap();
    let entry = r
        .get(RegistryLookupRequest {
            id: "k".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.as_deref(), Some("v"));
}
