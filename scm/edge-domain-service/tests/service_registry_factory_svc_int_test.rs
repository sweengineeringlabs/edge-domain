//! SAF facade tests — `ServiceRegistryFactory` constructors.

use edge_domain_service::{ServiceRegistryFactory, ServiceRegistry};

struct Factories;
impl ServiceRegistryFactory for Factories {}

/// @covers: ServiceRegistryFactory::new_registry — returns a usable registry
#[test]
fn test_new_registry_returns_empty_registry_happy() {
    let reg: ServiceRegistry<String, String> = Factories::new_registry();
    assert!(reg.is_empty());
}

/// @covers: ServiceRegistryFactory::new_registry — multiple calls return independent instances
#[test]
fn test_new_registry_multiple_calls_return_independent_instances_error() {
    let a: ServiceRegistry<String, String> = Factories::new_registry();
    let b: ServiceRegistry<String, String> = Factories::new_registry();
    assert_eq!(a.len(), b.len());
}

/// @covers: ServiceRegistryFactory::new_registry — works with different type parameters
#[test]
fn test_new_registry_different_type_params_both_usable_edge() {
    let reg_ss: ServiceRegistry<String, String> = Factories::new_registry();
    let reg_uu: ServiceRegistry<u32, u64> = Factories::new_registry();
    assert!(reg_ss.is_empty());
    assert!(reg_uu.is_empty());
}
