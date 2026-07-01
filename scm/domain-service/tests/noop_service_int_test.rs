//! Integration tests for [`NoopService`] — the no-operation service implementation.

use edge_domain_service::{NoopService, Service, NameRequest};
use futures::executor::block_on;

/// @covers: Service::name
#[test]
fn test_name_noop_service_returns_noop_happy() {
    let result = NoopService.name(NameRequest);
    assert_eq!(result.unwrap().name, "noop");
}

/// @covers: Service::execute
#[test]
fn test_execute_noop_service_returns_ok_happy() {
    let result = block_on(NoopService.execute(()));
    assert_eq!(result, Ok(()));
}

/// @covers: Service::execute
#[test]
fn test_execute_noop_service_repeated_always_succeeds_edge() {
    for _ in 0..3 {
        assert_eq!(block_on(NoopService.execute(())), Ok(()));
    }
}
