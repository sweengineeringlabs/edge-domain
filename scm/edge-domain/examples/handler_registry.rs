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
use edge_domain::{Domain, Handler, HandlerContext, HandlerError};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;

struct GreetHandler;

#[async_trait]
impl Handler for GreetHandler {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "greet"
    }
    fn pattern(&self) -> &str {
        "direct"
    }

    async fn execute(&self, req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        if req.is_empty() {
            return Err(HandlerError::InvalidRequest(
                "name must not be empty".into(),
            ));
        }
        Ok(format!("Hello, {req}!"))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Domain::new_handler_registry::<String, String>();
    assert!(registry.is_empty());

    registry.register(Arc::new(GreetHandler));
    println!("registered:   {:?}", registry.list_ids());

    let handler = registry.get("greet").expect("handler must be present");
    let security = SecurityContext::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, bus.as_ref(), observer.as_ref());

    let resp = handler.execute("world".into(), ctx).await?;
    println!("execute       → {resp}");

    let err = handler.execute("".into(), ctx).await.unwrap_err();
    println!("empty name    → {err}");

    let healthy = handler.health_check().await;
    println!("health_check  → {healthy}");
    assert!(healthy);

    let removed = registry.deregister("greet");
    assert!(removed);
    assert!(registry.get("greet").is_none());
    println!("after remove: {:?}", registry.list_ids());

    Ok(())
}
