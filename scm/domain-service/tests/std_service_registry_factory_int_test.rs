//! Tests for [`StdServiceRegistryFactory`] — default registry factory.

use edge_application_service::{Service, ServiceRegistry, StdServiceRegistryFactory};
use futures::executor::block_on;

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_is_copy() {
    let factory1 = StdServiceRegistryFactory;
    let factory2 = StdServiceRegistryFactory;
    assert_eq!(factory1, factory2);
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_debug_impl() {
    let factory = StdServiceRegistryFactory;
    let debug_str = format!("{:?}", factory);
    assert_eq!(debug_str, "StdServiceRegistryFactory");
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_default_impl() {
    let factory = StdServiceRegistryFactory::default();
    assert_eq!(factory, StdServiceRegistryFactory);
}

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_clone_impl() {
    let factory1 = StdServiceRegistryFactory;
    let factory2 = factory1.clone();
    assert_eq!(factory1, factory2);
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

/// @covers: StdServiceRegistryFactory
#[test]
fn test_std_service_registry_factory_produces_valid_registry_edge() {
    let registry = StdServiceRegistryFactory::new_registry::<String, String>();
    let req = edge_application_service::EmptinessRequest;
    match registry.is_empty(req) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}
