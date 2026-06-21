//! Integration tests for the `Handler` trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Domain, Handler, HandlerContext, HandlerError};
use edge_domain_observe::{ObserveContext, StdObserveFactory};
use edge_domain_security::SecurityContext;

struct Counter {
    id: String,
    calls: std::sync::atomic::AtomicUsize,
}

#[async_trait]
impl Handler for Counter {
    type Request = u32;
    type Response = u32;
    fn id(&self) -> &str {
        &self.id
    }
    fn pattern(&self) -> &str {
        "counter"
    }
    async fn execute(&self, req: u32, _ctx: HandlerContext<'_>) -> Result<u32, HandlerError> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(req * 2)
    }
}

struct SickHandler;
#[async_trait]
impl Handler for SickHandler {
    type Request = u32;
    type Response = u32;
    fn id(&self) -> &str {
        "sick"
    }
    fn pattern(&self) -> &str {
        "sick"
    }
    async fn execute(&self, _: u32, _ctx: HandlerContext<'_>) -> Result<u32, HandlerError> {
        Err(HandlerError::Unhealthy)
    }
    async fn health_check(&self) -> bool {
        false
    }
}

fn make_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a Arc<dyn edge_domain::CommandBus>,
    observer: &'a dyn ObserveContext,
) -> HandlerContext<'a> {
    HandlerContext::new(security, bus.as_ref(), observer)
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_handler_trait_execute_returns_transformed_value() {
    let h = Counter {
        id: "ctr".into(),
        calls: Default::default(),
    };
    let security = SecurityContext::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observe_context();
    let result = h
        .execute(21, make_ctx(&security, &bus, observer.as_ref()))
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
    assert!(h.health_check().await);
}

/// @covers: Handler::health_check — override to false
#[tokio::test]
async fn test_handler_trait_health_check_override_returns_false() {
    let h: Arc<dyn Handler<Request = u32, Response = u32>> = Arc::new(SickHandler);
    assert!(!h.health_check().await);
}
