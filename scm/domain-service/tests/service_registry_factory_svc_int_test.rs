//! Factory constructor tests — `StdServiceRegistryFactory` static methods.

use edge_domain_service::{
    NoopService, Service, ServiceRegistry, ServiceRegistryStore, StdServiceRegistryFactory,
};
use futures::executor::block_on;

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_returns_empty_registry_happy() {
    let reg: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let req = edge_domain_service::EmptinessRequest;
    assert!(reg.is_empty(req).unwrap().empty);
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_multiple_calls_return_independent_instances_edge() {
    let a: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let b: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let req = edge_domain_service::LenRequest;
    assert_eq!(a.len(req.clone()).unwrap().count, b.len(req).unwrap().count);
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_different_type_params_both_usable_edge() {
    let reg_ss: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let reg_uu: ServiceRegistryStore<u32, u64> = StdServiceRegistryFactory::new_registry();
    let req = edge_domain_service::EmptinessRequest;
    assert!(reg_ss.is_empty(req.clone()).unwrap().empty);
    assert!(reg_uu.is_empty(req).unwrap().empty);
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_returns_noop_service_instance_happy() {
    let svc: NoopService = StdServiceRegistryFactory::noop_service();
    let result = svc.name(edge_domain_service::NameRequest);
    assert_eq!(result.unwrap().name, "noop");
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
    let req = edge_domain_service::NameRequest;
    assert_eq!(
        a.name(req.clone()).unwrap().name,
        b.name(req).unwrap().name
    );
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let factory = StdServiceRegistryFactory::default_factory();
    let reg: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let _ = factory;
    let req = edge_domain_service::EmptinessRequest;
    assert!(reg.is_empty(req).unwrap().empty);
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_creates_usable_registry_happy() {
    let _factory = StdServiceRegistryFactory::default_factory();
    let reg: ServiceRegistryStore<u32, u32> = StdServiceRegistryFactory::new_registry();
    let req = edge_domain_service::EmptinessRequest;
    assert!(reg.is_empty(req).unwrap().empty);
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_multiple_calls_independent_edge() {
    let a = StdServiceRegistryFactory::default_factory();
    let b = StdServiceRegistryFactory::default_factory();
    let reg_a: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let reg_b: ServiceRegistryStore<String, String> = StdServiceRegistryFactory::new_registry();
    let req = edge_domain_service::LenRequest;
    assert_eq!(reg_a.len(req.clone()).unwrap().count, reg_b.len(req).unwrap().count);
    let _ = (a, b);
}
