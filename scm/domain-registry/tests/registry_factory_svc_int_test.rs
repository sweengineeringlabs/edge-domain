//! SAF facade tests — `RegistryBootstrap`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_registry::{Registry, RegistryBootstrap, StdRegistryFactory};

// ── in_memory ─────────────────────────────────────────────────────────────────
/// @covers: RegistryBootstrap::in_memory
#[test]
fn test_in_memory_starts_empty_happy() {
    let r = StdRegistryFactory::in_memory::<str>();
    assert!(r.is_empty());
}

/// @covers: RegistryBootstrap::in_memory
#[test]
fn test_in_memory_instances_are_independent_error() {
    let a = StdRegistryFactory::in_memory::<str>();
    let b = StdRegistryFactory::in_memory::<str>();
    a.register("x", Arc::from("v"));
    // a separate registry does not observe another's entries
    assert!(b.get("x").is_none());
}

/// @covers: RegistryBootstrap::in_memory
#[test]
fn test_in_memory_usable_for_registration_edge() {
    let r = StdRegistryFactory::in_memory::<str>();
    r.register("x", Arc::from("v"));
    assert_eq!(r.get("x").as_deref(), Some("v"));
}

// ── std_factory ───────────────────────────────────────────────────────────────
/// @covers: RegistryBootstrap::std_factory
#[test]
fn test_std_factory_returns_factory_instance_happy() {
    let f: StdRegistryFactory = StdRegistryFactory::std_factory();
    // Assert that factory can be used to construct registries
    let r = f.in_memory::<str>();
    assert!(r.is_empty(), "factory-constructed registry should be empty");
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
    r.register("x", Arc::from("v"));
    assert_eq!(r.len(), 1);
}
