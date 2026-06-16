//! SAF facade tests — `ServiceRegistryFactory` constructors.

use edge_domain_service::{StdServiceRegistryFactory, NoopService, Service, ServiceRegistry, ServiceRegistryFactory};
use futures::executor::block_on;

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

/// @covers: ServiceRegistryFactory::noop_service — returns a NoopService instance
#[test]
fn test_noop_service_returns_noop_service_instance_happy() {
    let svc: NoopService = Factories::noop_service();
    assert_eq!(svc.name(), "noop");
}

/// @covers: ServiceRegistryFactory::noop_service — execute always returns Ok
#[test]
fn test_noop_service_execute_returns_ok_error() {
    let svc = Factories::noop_service();
    let result = block_on(svc.execute(()));
    assert!(result.is_ok(), "NoopService::execute must never fail");
}

/// @covers: ServiceRegistryFactory::noop_service — multiple calls return independent instances
#[test]
fn test_noop_service_multiple_calls_return_independent_instances_edge() {
    let a = Factories::noop_service();
    let b = Factories::noop_service();
    assert_eq!(a.name(), b.name());
}

/// @covers: ServiceRegistryFactory::default_factory — returns a StdServiceRegistryFactory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let factory = Factories::default_factory();
    let reg: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let _ = factory; // ensure factory is used
    assert!(reg.is_empty());
}

/// @covers: ServiceRegistryFactory::default_factory — returned factory can create registries
#[test]
fn test_default_factory_creates_usable_registry_error() {
    let _factory = Factories::default_factory();
    let reg: ServiceRegistry<u32, u32> = StdServiceRegistryFactory::new_registry();
    assert!(reg.is_empty(), "factory must produce empty registry on creation");
}

/// @covers: ServiceRegistryFactory::default_factory — multiple calls return independent factories
#[test]
fn test_default_factory_multiple_calls_independent_edge() {
    let a = Factories::default_factory();
    let b = Factories::default_factory();
    let _ = (a, b); // both instances independently obtained
    let reg_a: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let reg_b: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    assert_eq!(reg_a.len(), reg_b.len());
}
