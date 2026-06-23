#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — HandlerBootstrap trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::Handler;
use edge_domain::HandlerBootstrap;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;

struct Cfg {
    label: String,
}

struct LabelHandler {
    label: String,
}

impl HandlerBootstrap for LabelHandler {
    type Config = Cfg;
    fn build(cfg: Cfg) -> Result<Self, HandlerError> {
        Ok(LabelHandler { label: cfg.label })
    }
}

#[async_trait]
impl Handler for LabelHandler {
    type Request = ();
    type Response = String;
    fn id(&self) -> &str {
        &self.label
    }
    fn pattern(&self) -> &str {
        "*"
    }
    async fn execute(&self, _: (), _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        Ok(self.label.clone())
    }
}

#[test]
fn test_handler_factory_svc_facade_build_constructs_handler() {
    let h = LabelHandler::build(Cfg {
        label: "greet".into(),
    })
    .unwrap();
    assert_eq!(h.id(), "greet");
}

#[tokio::test]
async fn test_handler_factory_svc_facade_built_handler_executes() {
    let h = LabelHandler::build(Cfg {
        label: "echo".into(),
    })
    .unwrap();
    let security = SecurityContext::unauthenticated();
    let bus = edge_domain::Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, bus.as_ref(), observer.as_ref());
    assert_eq!(h.execute((), ctx).await.unwrap(), "echo");
}
