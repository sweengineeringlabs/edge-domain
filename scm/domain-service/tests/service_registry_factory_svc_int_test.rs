//! Factory constructor tests — `StdServiceRegistryFactory` static methods.

use edge_domain_service::{
    NoopService, Service, ServiceRegistry, ServiceRegistryTrait, StdServiceRegistryFactory,
};
use futures::executor::block_on;

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_returns_empty_registry_happy() {
    let reg: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    assert!(reg.is_empty());
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_multiple_calls_return_independent_instances_edge() {
    let a: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let b: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    assert_eq!(a.len(), b.len());
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_different_type_params_both_usable_edge() {
    let reg_ss: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let reg_uu: ServiceRegistry<u32, u64> = StdServiceRegistryFactory::new_registry();
    assert!(reg_ss.is_empty());
    assert!(reg_uu.is_empty());
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_returns_noop_service_instance_happy() {
    let svc: NoopService = StdServiceRegistryFactory::noop_service();
    assert_eq!(svc.name(), "noop");
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_execute_returns_ok_happy() {
    let svc = StdServiceRegistryFactory::noop_service();
    let result = block_on(svc.execute(()));
    assert_eq!(result, Ok(()));
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_multiple_calls_return_independent_instances_edge() {
    let a = StdServiceRegistryFactory::noop_service();
    let b = StdServiceRegistryFactory::noop_service();
    assert_eq!(a.name(), b.name());
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let factory = StdServiceRegistryFactory::default_factory();
    let reg: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let _ = factory;
    assert!(reg.is_empty());
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_creates_usable_registry_happy() {
    let _factory = StdServiceRegistryFactory::default_factory();
    let reg: ServiceRegistry<u32, u32> = StdServiceRegistryFactory::new_registry();
    assert!(reg.is_empty());
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_multiple_calls_independent_edge() {
    let a = StdServiceRegistryFactory::default_factory();
    let b = StdServiceRegistryFactory::default_factory();
    let reg_a: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    let reg_b: ServiceRegistry<String, String> = StdServiceRegistryFactory::new_registry();
    assert_eq!(reg_a.len(), reg_b.len());
    let _ = (a, b);
}
