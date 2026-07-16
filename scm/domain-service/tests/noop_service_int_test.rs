//! Integration tests for [`NoopService`] — the no-operation service implementation.

use edge_application_service::{NameRequest, NoopRequest, NoopResponse, NoopService, Service};
use futures::executor::block_on;

/// @covers: Service::name
#[test]
fn test_name_noop_service_returns_noop_happy() {
    let result = NoopService.name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "noop"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: Service::execute
#[test]
fn test_execute_noop_service_returns_ok_happy() {
    let result = block_on(NoopService.execute(NoopRequest));
    assert_eq!(result, Ok(NoopResponse));
}

/// @covers: Service::execute
#[test]
fn test_execute_noop_service_repeated_always_succeeds_edge() {
    for _ in 0..3 {
        assert_eq!(block_on(NoopService.execute(NoopRequest)), Ok(NoopResponse));
    }
}
