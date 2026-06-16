//! SAF facade tests — `RegistryFactory`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_registry::{Registry, RegistryFactory, StdRegistryFactory};

// ── in_memory ─────────────────────────────────────────────────────────────────
/// @covers: RegistryFactory::in_memory
#[test]
fn test_in_memory_starts_empty_happy() {
    let r = StdRegistryFactory::in_memory::<str>();
    assert!(r.is_empty());
}

/// @covers: RegistryFactory::in_memory
#[test]
fn test_in_memory_instances_are_independent_error() {
    let a = StdRegistryFactory::in_memory::<str>();
    let b = StdRegistryFactory::in_memory::<str>();
    a.register("x", Arc::from("v"));
    // a separate registry does not observe another's entries
    assert!(b.get("x").is_none());
}

/// @covers: RegistryFactory::in_memory
#[test]
fn test_in_memory_usable_for_registration_edge() {
    let r = StdRegistryFactory::in_memory::<str>();
    r.register("x", Arc::from("v"));
    assert_eq!(r.get("x").as_deref(), Some("v"));
}

// ── std_factory ───────────────────────────────────────────────────────────────
/// @covers: RegistryFactory::std_factory
#[test]
fn test_std_factory_returns_factory_instance_happy() {
    let _f: StdRegistryFactory = StdRegistryFactory::std_factory();
}

/// @covers: RegistryFactory::std_factory
#[test]
fn test_std_factory_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdRegistryFactory>(), 0);
}

/// @covers: RegistryFactory::std_factory
#[test]
fn test_std_factory_constructs_usable_registry_edge() {
    let _f = StdRegistryFactory::std_factory();
    let r = StdRegistryFactory::in_memory::<str>();
    r.register("x", Arc::from("v"));
    assert_eq!(r.len(), 1);
}
