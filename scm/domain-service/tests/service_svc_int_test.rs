//! SAF facade tests — `Service` trait.

use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Echo(String);
impl Service for Echo {
    type Request = String;
    type Response = String;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: self.0.clone(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct AlwaysFails;
impl Service for AlwaysFails {
    type Request = String;
    type Response = String;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "service".to_string(),
        })
    }

    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async { Err(ServiceError::RuleViolation("blocked".into())) })
    }
}

/// @covers: Service::name
#[test]
fn test_name_configured_value_returned_happy() {
    let result = Echo("greet".into()).name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "greet"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: Service::name
#[test]
fn test_name_default_impl_returns_service_happy() {
    let result = AlwaysFails.name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "service"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: Service::name — via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_returns_name_edge() {
    let svc: &dyn Service<Request = String, Response = String> = &Echo("ping".into());
    let result = svc.name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "ping"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: Service::execute — success path returns value
#[test]
fn test_execute_ok_service_returns_response_happy() {
    let result = block_on(Echo("echo".into()).execute("hello".into()));
    assert_eq!(result.ok().as_deref(), Some("hello"));
}

/// @covers: Service::execute — failure propagates error
#[test]
fn test_execute_failing_service_returns_err_error() {
    let result = block_on(AlwaysFails.execute("any".into()));
    assert!(result.is_err());
}

/// @covers: Service::execute — repeated calls are independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let svc = Echo("e".into());
    assert_eq!(block_on(svc.execute("a".into())).ok(), Some("a".into()));
    assert_eq!(block_on(svc.execute("b".into())).ok(), Some("b".into()));
}
