//! Comprehensive tests for Service trait methods.

use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::executor::block_on;
use futures::future::BoxFuture;

/// A test double whose `name()` always fails, used to exercise the error path
/// of the `Service` trait contract (NoopService never errors, so it can't
/// cover this scenario on its own).
struct FailingService;

impl Service for FailingService {
    type Request = ();
    type Response = ();

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Err(ServiceError::NotFound("failing-service".to_string()))
    }

    fn execute(&self, _req: ()) -> BoxFuture<'_, Result<(), ServiceError>> {
        Box::pin(async move { Err(ServiceError::NotFound("failing-service".to_string())) })
    }
}

/// @covers: Service::name
#[test]
fn test_name_returns_noop_happy() {
    use edge_domain_service::NoopService;
    let result = NoopService.name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "noop"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: Service::name
#[test]
fn test_name_consistent_edge() {
    use edge_domain_service::NoopService;
    let r1 = NoopService.name(NameRequest);
    let r2 = NoopService.name(NameRequest);
    assert_eq!(r1, r2);
}

/// @covers: Service::name
#[test]
fn test_name_failing_service_returns_err_error() {
    let result = FailingService.name(NameRequest);
    assert!(result.is_err());
}

/// @covers: Service::execute
#[test]
fn test_execute_returns_ok_happy() {
    use edge_domain_service::NoopService;
    let result = block_on(NoopService.execute(()));
    assert_eq!(result, Ok(()));
}

/// @covers: Service::execute
#[test]
fn test_execute_idempotent_edge() {
    use edge_domain_service::NoopService;
    for _ in 0..5 {
        let result = block_on(NoopService.execute(()));
        assert_eq!(result, Ok(()));
    }
}

/// @covers: Service::execute
#[test]
fn test_execute_failing_returns_err_error() {
    let result = block_on(FailingService.execute(()));
    assert!(result.is_err());
}
