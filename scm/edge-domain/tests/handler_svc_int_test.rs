#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Handler trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::Domain;
use edge_domain::Handler;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;
use edge_domain_handler::{ExecutionRequest, HealthCheckRequest, IdRequest, IdResponse};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};

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
    let security = SecurityServices::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
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
