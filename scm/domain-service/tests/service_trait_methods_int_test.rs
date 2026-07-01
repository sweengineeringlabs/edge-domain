//! Comprehensive tests for Service trait methods via NoopService.

use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::executor::block_on;

/// @covers: Service::name
#[test]
fn test_service_name_returns_noop_happy() {
    use edge_domain_service::NoopService;
    let result = NoopService.name(NameRequest);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "noop");
}

/// @covers: Service::name
#[test]
fn test_service_name_consistent_edge() {
    use edge_domain_service::NoopService;
    let r1 = NoopService.name(NameRequest);
    let r2 = NoopService.name(NameRequest);
    assert_eq!(r1, r2);
}

/// @covers: Service::execute
#[test]
fn test_service_execute_returns_ok_happy() {
    use edge_domain_service::NoopService;
    let result = block_on(NoopService.execute(()));
    assert_eq!(result, Ok(()));
}

/// @covers: Service::execute
#[test]
fn test_service_execute_idempotent_edge() {
    use edge_domain_service::NoopService;
    for _ in 0..5 {
        let result = block_on(NoopService.execute(()));
        assert_eq!(result, Ok(()));
    }
}
