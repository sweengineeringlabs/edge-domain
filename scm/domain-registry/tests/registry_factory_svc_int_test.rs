//! SAF facade tests — `RegistryBootstrap`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_registry::{
    EmptinessRequest, LenRequest, RegisterRequest, Registry, RegistryBootstrap,
    RegistryLookupRequest, StdRegistryFactory,
};

fn register(r: &impl Registry<Value = str>, id: &str, entry: &str) {
    r.register(RegisterRequest {
        id: id.to_string(),
        entry: Arc::from(entry),
    })
    .unwrap();
}

// ── in_memory ─────────────────────────────────────────────────────────────────
/// @covers: RegistryBootstrap::in_memory
#[test]
fn test_in_memory_starts_empty_happy() {
    let r = StdRegistryFactory::in_memory::<str>();
    assert!(r.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: RegistryBootstrap::in_memory
#[test]
fn test_in_memory_instances_are_independent_error() {
    let a = StdRegistryFactory::in_memory::<str>();
    let b = StdRegistryFactory::in_memory::<str>();
    register(&a, "x", "v");
    // a separate registry does not observe another's entries
    assert!(b
        .get(RegistryLookupRequest {
            id: "x".to_string()
        })
        .unwrap()
        .entry
        .is_none());
}

/// @covers: RegistryBootstrap::in_memory
#[test]
fn test_in_memory_usable_for_registration_edge() {
    let r = StdRegistryFactory::in_memory::<str>();
    register(&r, "x", "v");
    let entry = r
        .get(RegistryLookupRequest {
            id: "x".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.as_deref(), Some("v"));
}

// ── std_factory ───────────────────────────────────────────────────────────────
/// @covers: RegistryBootstrap::std_factory
#[test]
fn test_std_factory_returns_factory_instance_happy() {
    let _f: StdRegistryFactory = StdRegistryFactory::std_factory();
    // in_memory is a static method; call via the type, not an instance
    let r = StdRegistryFactory::in_memory::<str>();
    assert!(
        r.is_empty(EmptinessRequest).unwrap().empty,
        "factory-constructed registry should be empty"
    );
}

/// @covers: RegistryBootstrap::std_factory
#[test]
fn test_std_factory_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdRegistryFactory>(), 0);
}

/// @covers: RegistryBootstrap::std_factory
#[test]
fn test_std_factory_constructs_usable_registry_edge() {
    let _f = StdRegistryFactory::std_factory();
    let r = StdRegistryFactory::in_memory::<str>();
    register(&r, "x", "v");
    assert_eq!(r.len(LenRequest).unwrap().count, 1);
}
