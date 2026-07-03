#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — HandlerBootstrap trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::Handler;
use edge_domain::HandlerBootstrap;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;
use edge_domain_handler::{ExecutionRequest, HandlerBuildResponse, IdRequest, IdResponse};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};

struct Cfg {
    label: String,
}

struct LabelHandler {
    label: String,
}

impl HandlerBootstrap for LabelHandler {
    type Config = Cfg;
    fn build(cfg: Cfg) -> Result<HandlerBuildResponse<Self>, HandlerError> {
        Ok(HandlerBuildResponse {
            handler: LabelHandler { label: cfg.label },
        })
    }
}

#[async_trait]
impl Handler for LabelHandler {
    type Request = ();
    type Response = String;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.label.clone(),
        })
    }
    async fn execute(&self, _req: ExecutionRequest<'_, ()>) -> Result<String, HandlerError> {
        Ok(self.label.clone())
    }
}

#[test]
fn test_handler_factory_svc_facade_build_constructs_handler() {
    let h = LabelHandler::build(Cfg {
        label: "greet".into(),
    })
    .unwrap()
    .handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "greet");
}

#[tokio::test]
async fn test_handler_factory_svc_facade_built_handler_executes() {
    let h = LabelHandler::build(Cfg {
        label: "echo".into(),
    })
    .unwrap()
    .handler;
    let security = SecurityServices::unauthenticated();
    let bus = edge_domain::Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
    };
    assert_eq!(
        h.execute(ExecutionRequest { req: (), ctx: &ctx })
            .await
            .unwrap(),
        "echo"
    );
}
