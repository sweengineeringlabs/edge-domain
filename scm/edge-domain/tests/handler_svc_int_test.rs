#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Handler trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::Domain;
use edge_domain::Handler;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;

struct Doubler;

#[async_trait]
impl Handler for Doubler {
    type Request = i32;
    type Response = i32;
    fn id(&self) -> &str {
        "doubler"
    }
    fn pattern(&self) -> &str {
        "*"
    }
    async fn execute(&self, req: i32, _ctx: HandlerContext<'_>) -> Result<i32, HandlerError> {
        Ok(req * 2)
    }
}

#[tokio::test]
async fn test_handler_svc_facade_execute_doubles_input() {
    let security = SecurityContext::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, bus.as_ref(), observer.as_ref());
    assert_eq!(Doubler.execute(21, ctx).await.unwrap(), 42);
}

#[tokio::test]
async fn test_handler_svc_facade_health_check_defaults_true() {
    assert!(Doubler.health_check().await);
}
