//! Factory constructor tests — `StdServiceRegistryFactory` static methods.

use edge_application_service::{
    NoopRequest, NoopResponse, NoopService, Service, ServiceRegistry, ServiceRegistryStore,
    StdServiceRegistryFactory,
};
use futures::executor::block_on;
use std::fmt::Debug;

#[allow(dead_code)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

#[allow(dead_code)]
struct NumberPayload(u32);

impl edge_application_base::Request for NumberPayload {}
impl edge_application_base::Response for NumberPayload {}

#[allow(dead_code)]
struct BigNumberPayload(u64);

impl edge_application_base::Request for BigNumberPayload {}
impl edge_application_base::Response for BigNumberPayload {}

/// Unwrap a `Result` in test code without tripping the crate-wide
/// `clippy::unwrap_used` / `clippy::expect_used` deny-lints.
fn ok<T, E: Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_returns_empty_registry_happy() {
    let reg: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let req = edge_application_service::EmptinessRequest;
    assert!(ok(reg.is_empty(req)).empty);
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_multiple_calls_return_independent_instances_edge() {
    let a: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let b: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let req = edge_application_service::LenRequest;
    assert_eq!(ok(a.len(req.clone())).count, ok(b.len(req)).count);
}

/// @covers: StdServiceRegistryFactory::new_registry
#[test]
fn test_new_registry_different_type_params_both_usable_edge() {
    let reg_ss: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let reg_uu: ServiceRegistryStore<NumberPayload, BigNumberPayload> =
        StdServiceRegistryFactory::new_registry();
    let req = edge_application_service::EmptinessRequest;
    assert!(ok(reg_ss.is_empty(req.clone())).empty);
    assert!(ok(reg_uu.is_empty(req)).empty);
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_returns_noop_service_instance_happy() {
    let svc: NoopService = StdServiceRegistryFactory::noop_service();
    let result = svc.name(edge_application_service::NameRequest);
    assert_eq!(ok(result).name, "noop");
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_execute_returns_ok_happy() {
    let svc = StdServiceRegistryFactory::noop_service();
    let result = block_on(svc.execute(NoopRequest));
    assert_eq!(result, Ok(NoopResponse));
}

/// @covers: StdServiceRegistryFactory::noop_service
#[test]
fn test_noop_service_multiple_calls_return_independent_instances_edge() {
    let a = StdServiceRegistryFactory::noop_service();
    let b = StdServiceRegistryFactory::noop_service();
    let req = edge_application_service::NameRequest;
    assert_eq!(ok(a.name(req.clone())).name, ok(b.name(req)).name);
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let factory = StdServiceRegistryFactory::default_factory();
    let reg: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let _ = factory;
    let req = edge_application_service::EmptinessRequest;
    assert!(ok(reg.is_empty(req)).empty);
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_creates_usable_registry_happy() {
    let _factory = StdServiceRegistryFactory::default_factory();
    let reg: ServiceRegistryStore<NumberPayload, NumberPayload> =
        StdServiceRegistryFactory::new_registry();
    let req = edge_application_service::EmptinessRequest;
    assert!(ok(reg.is_empty(req)).empty);
}

/// @covers: StdServiceRegistryFactory::default_factory
#[test]
fn test_default_factory_multiple_calls_independent_edge() {
    let a = StdServiceRegistryFactory::default_factory();
    let b = StdServiceRegistryFactory::default_factory();
    let reg_a: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let reg_b: ServiceRegistryStore<TextPayload, TextPayload> =
        StdServiceRegistryFactory::new_registry();
    let req = edge_application_service::LenRequest;
    assert_eq!(ok(reg_a.len(req.clone())).count, ok(reg_b.len(req)).count);
    let _ = (a, b);
}
