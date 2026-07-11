#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Handler trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::DirectCommandBusRequest;
use edge_domain::Domain;
use edge_domain::DomainRuntime;
use edge_domain::Handler;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;
use edge_domain_handler::{
    CommandBusAdapter, ExecutionRequest, HealthCheckRequest, IdRequest, IdResponse,
    ObserverContextAdapter,
};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

struct Doubler;

#[async_trait]
impl Handler for Doubler {
    type Request = i32;
    type Response = i32;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "doubler".to_string(),
        })
    }
    async fn execute(&self, req: ExecutionRequest<'_, i32>) -> Result<i32, HandlerError> {
        Ok(req.req * 2)
    }
}

#[tokio::test]
async fn test_handler_svc_facade_execute_doubles_input() {
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let bus_adapter = CommandBusAdapter(bus.as_ref());
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = HandlerContext {
        security: &security,
        commands: &bus_adapter,
        observer: &observer_adapter,
    };
    assert_eq!(
        Doubler
            .execute(ExecutionRequest { req: 21, ctx: &ctx })
            .await
            .unwrap(),
        42
    );
}

#[tokio::test]
async fn test_handler_svc_facade_health_check_defaults_true() {
    assert!(
        Doubler
            .health_check(HealthCheckRequest)
            .await
            .unwrap()
            .healthy
    );
}
