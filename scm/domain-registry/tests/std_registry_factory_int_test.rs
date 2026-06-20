//! Integration tests for `StdRegistryFactory` — covers the types/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::{RegistryBootstrap, StdRegistryFactory};

/// @covers: StdRegistryFactory — constructs via literal and `std_factory`
#[test]
fn test_std_registry_factory_constructs_happy() {
    let _f = StdRegistryFactory;
    let _g = StdRegistryFactory::std_factory();
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
    let _g = f; // Copy
    let _h = f; // still usable after copy
    let _ = StdRegistryFactory::std_factory();
}
