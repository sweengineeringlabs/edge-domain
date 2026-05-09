//! Handler registry — register, resolve, execute, and deregister domain Handlers.
//!
//! Run:
//!     cargo run -p edge-domain --example handler_registry
//!
//! Demonstrates the full domain execution-unit contract:
//!   new_handler_registry → register → get → execute → health_check → deregister
//!
//! SEA constraint: all imports come from the `edge_domain` SAF surface.

use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError, RequestContext, new_handler_registry};

struct GreetHandler;

#[async_trait]
impl Handler<String, String> for GreetHandler {
    fn id(&self)      -> &str { "greet" }
    fn pattern(&self) -> &str { "direct" }

    async fn execute(&self, req: String, _ctx: RequestContext) -> Result<String, HandlerError> {
        if req.is_empty() {
            return Err(HandlerError::InvalidRequest("name must not be empty".into()));
        }
        Ok(format!("Hello, {req}!"))
    }

    async fn health_check(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = new_handler_registry::<String, String>();
    assert!(registry.is_empty());

    registry.register(Arc::new(GreetHandler));
    println!("registered:   {:?}", registry.list_ids());

    let handler = registry.get("greet").expect("handler must be present");
    let ctx     = RequestContext::unauthenticated();
    let resp    = handler.execute("world".into(), ctx.clone()).await?;
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
