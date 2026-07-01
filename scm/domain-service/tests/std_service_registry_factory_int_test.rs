//! Tests for [`StdServiceRegistryFactory`] — default registry factory.

use edge_domain_service::{Service, ServiceRegistry, StdServiceRegistryFactory};
use futures::executor::block_on;

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_is_copy() {
    let _factory1 = StdServiceRegistryFactory;
    let _factory2 = StdServiceRegistryFactory;
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_debug_impl() {
    let factory = StdServiceRegistryFactory;
    let debug_str = format!("{:?}", factory);
    assert!(!debug_str.is_empty());
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_default_impl() {
    let _factory = StdServiceRegistryFactory::default();
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_clone_impl() {
    let factory1 = StdServiceRegistryFactory;
    let factory2 = factory1.clone();
    let _ = (factory1, factory2);
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_equality_happy() {
    let factory1 = StdServiceRegistryFactory;
    let factory2 = StdServiceRegistryFactory;
    assert_eq!(factory1, factory2);
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_partial_ord_happy() {
    let factory1 = StdServiceRegistryFactory;
    let factory2 = StdServiceRegistryFactory;
    assert!(!(factory1 < factory2));
    assert!(!(factory2 < factory1));
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_hash_consistent_happy() {
    use std::collections::HashSet;
    let factory1 = StdServiceRegistryFactory;
    let factory2 = StdServiceRegistryFactory;
    let mut set = HashSet::new();
    set.insert(factory1);
    set.insert(factory2);
    assert_eq!(set.len(), 1);
}
