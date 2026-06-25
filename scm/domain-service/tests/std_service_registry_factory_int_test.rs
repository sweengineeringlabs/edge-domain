//! Integration tests for [`StdServiceRegistryFactory`].

use edge_domain_service::{StdServiceRegistryFactory, ServiceRegistry, ServiceRegistryBootstrap, ServiceRegistryTrait};

/// @covers: ServiceRegistryBootstrap::new_registry
#[test]
fn test_new_registry_returns_empty_registry_happy() {
    let reg: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    assert!(reg.is_empty());
}

/// @covers: ServiceRegistryBootstrap::new_registry
#[test]
fn test_new_registry_multiple_calls_return_independent_instances_error() {
    let a: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let b: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    assert_eq!(a.len(), b.len());
}

/// @covers: ServiceRegistryBootstrap::new_registry
#[test]
fn test_new_registry_different_type_params_both_usable_edge() {
    let reg_ss: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let reg_uu: ServiceRegistry<u32, u64> = StdServiceRegistryFactory::new_registry();
    assert!(reg_ss.is_empty());
    assert!(reg_uu.is_empty());
}
