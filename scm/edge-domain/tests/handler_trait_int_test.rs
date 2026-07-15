//! Integration tests for the `Handler` trait contract.
#![cfg(all(feature = "command", feature = "handler"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application::DirectCommandBusRequest;
use edge_application::DomainRuntime;
use edge_application::{Domain, Handler, HandlerContext, HandlerError};
use edge_application_handler::{
    CommandBusAdapter, ExecutionRequest, HealthCheckRequest, IdRequest, IdResponse,
    ObserverContextAdapter,
};
use edge_application_observer::{ObserverContext, StdObserveFactory};
use edge_security_runtime::SecurityContext;

struct Counter {
    id: String,
    calls: std::sync::atomic::AtomicUsize,
}

#[async_trait]
impl Handler for Counter {
    type Request = u32;
    type Response = u32;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.clone(),
        })
    }
    async fn execute(&self, req: ExecutionRequest<'_, u32>) -> Result<u32, HandlerError> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(req.req * 2)
    }
}

struct SickHandler;
#[async_trait]
impl Handler for SickHandler {
    type Request = u32;
    type Response = u32;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "sick".to_string(),
        })
    }
    async fn execute(&self, _req: ExecutionRequest<'_, u32>) -> Result<u32, HandlerError> {
        Err(HandlerError::Unhealthy)
    }
    async fn health_check(
        &self,
        _req: HealthCheckRequest,
    ) -> Result<edge_application_handler::HealthCheckResponse, HandlerError> {
        Ok(edge_application_handler::HealthCheckResponse { healthy: false })
    }
}

fn make_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a CommandBusAdapter<'a, dyn edge_application::CommandBus>,
    observer: &'a ObserverContextAdapter<'a, dyn ObserverContext>,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus,
        observer,
    }
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_handler_trait_execute_returns_transformed_value() {
    let h = Counter {
        id: "ctr".into(),
        calls: Default::default(),
    };
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let bus_erased: &dyn edge_application::CommandBus = bus.as_ref();
    let bus_adapter = CommandBusAdapter(bus_erased);
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = make_ctx(&security, &bus_adapter, &observer_adapter);
    let result = h
        .execute(ExecutionRequest { req: 21, ctx: &ctx })
        .await
        .unwrap();
    assert_eq!(result, 42);
}

/// @covers: Handler::health_check — default is true
#[tokio::test]
async fn test_handler_trait_health_check_defaults_to_true() {
    let h = Counter {
        id: "ctr".into(),
        calls: Default::default(),
    };
    assert!(h.health_check(HealthCheckRequest).await.unwrap().healthy);
}

/// @covers: Handler::health_check — override to false
#[tokio::test]
async fn test_handler_trait_health_check_override_returns_false() {
    let h: Arc<dyn Handler<Request = u32, Response = u32>> = Arc::new(SickHandler);
    assert!(!h.health_check(HealthCheckRequest).await.unwrap().healthy);
}
