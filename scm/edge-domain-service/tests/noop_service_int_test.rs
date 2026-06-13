//! Integration tests for [`NoopService`] — the no-operation service implementation.

use edge_domain_service::{NoopService, Service};
use futures::executor::block_on;

/// @covers: Service::name
#[test]
fn test_name_noop_service_returns_noop_happy() {
    assert_eq!(NoopService.name(), "noop");
}

/// @covers: Service::execute
#[test]
fn test_execute_noop_service_returns_ok_happy() {
    let result = block_on(NoopService.execute(()));
    assert!(result.is_ok());
}

/// @covers: Service::execute
#[test]
fn test_execute_noop_service_repeated_always_succeeds_edge() {
    for _ in 0..3 {
        assert!(block_on(NoopService.execute(())).is_ok());
    }
}
