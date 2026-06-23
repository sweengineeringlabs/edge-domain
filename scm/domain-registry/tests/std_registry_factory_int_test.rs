//! Integration tests for `StdRegistryFactory` — covers the types/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::{RegistryBootstrap, StdRegistryFactory};

/// @covers: StdRegistryFactory — constructs via literal and `std_factory`
#[test]
fn test_std_registry_factory_constructs_happy() {
    let f = StdRegistryFactory;
    let g = StdRegistryFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0, "factory via literal is ZST");
    assert_eq!(std::mem::size_of_val(&g), 0, "factory via std_factory is ZST");
}

/// @covers: StdRegistryFactory — zero-sized marker type
#[test]
fn test_std_registry_factory_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdRegistryFactory>(), 0);
}

/// @covers: StdRegistryFactory — `std_factory` returns the factory; type is `Copy`
#[test]
fn test_std_registry_factory_copy_and_std_factory_edge() {
    let f = StdRegistryFactory;
    let g = f; // Copy
    let h = f; // still usable after copy
    let registry = StdRegistryFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0, "factory is ZST");
    assert_eq!(std::mem::size_of_val(&g), 0, "copy is ZST");
    assert_eq!(std::mem::size_of_val(&h), 0, "factory after copy is ZST");
    assert_eq!(std::mem::size_of_val(&registry), 0, "std_factory returns ZST");
}
