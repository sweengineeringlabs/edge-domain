//! Handler registry — register, resolve, execute, and deregister domain Handlers.
//!
//! Run:
//!     cargo run -p edge-domain --example handler_registry
//!
//! Demonstrates the full domain execution-unit contract:
//!   new_handler_registry → register → get → execute → health_check → deregister
//!
//! SEA constraint: all imports come from the `edge_domain` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::DirectCommandBusRequest;
use edge_domain::DomainRuntime;
use edge_domain::{Domain, Handler, HandlerContext, HandlerError};
use edge_domain_handler::{
    CommandBusAdapter, DeregisterHandlerRequest, EmptinessRequest, ExecutionRequest,
    HandlerLookupRequest, IdRequest, IdResponse, ListIdsRequest, ObserverContextAdapter,
    RegisterHandlerRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

struct GreetHandler;

#[async_trait]
impl Handler for GreetHandler {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "greet".to_string(),
        })
    }

    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        if req.req.is_empty() {
            return Err(HandlerError::InvalidRequest(
                "name must not be empty".into(),
            ));
        }
        Ok(format!("Hello, {}!", req.req))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Domain.new_handler_registry::<String, String>();
    assert!(registry.is_empty(EmptinessRequest)?.empty);

    registry.register(RegisterHandlerRequest::new(Arc::new(GreetHandler)))?;
    println!("registered:   {:?}", registry.list_ids(ListIdsRequest)?.ids);

    let handler = registry
        .get(HandlerLookupRequest {
            id: "greet".to_string(),
        })?
        .handler
        .expect("handler must be present");
    let security = SecurityContext::unauthenticated();
    let bus = Domain.direct_command_bus(DirectCommandBusRequest)?.bus;
    let bus_adapter = CommandBusAdapter(bus.as_ref());
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = HandlerContext {
        security: &security,
        commands: &bus_adapter,
        observer: &observer_adapter,
    };

    let resp = handler
        .execute(ExecutionRequest {
            req: "world".into(),
            ctx: &ctx,
        })
        .await?;
    println!("execute       → {resp}");

    let err = handler
        .execute(ExecutionRequest {
            req: "".into(),
            ctx: &ctx,
        })
        .await
        .unwrap_err();
    println!("empty name    → {err}");

    let healthy = handler
        .health_check(edge_domain_handler::HealthCheckRequest)
        .await?
        .healthy;
    println!("health_check  → {healthy}");
    assert!(healthy);

    let removed = registry
        .deregister(DeregisterHandlerRequest {
            id: "greet".to_string(),
        })?
        .was_present;
    assert!(removed);
    assert!(registry
        .get(HandlerLookupRequest {
            id: "greet".to_string()
        })?
        .handler
        .is_none());
    println!("after remove: {:?}", registry.list_ids(ListIdsRequest)?.ids);

    Ok(())
}
