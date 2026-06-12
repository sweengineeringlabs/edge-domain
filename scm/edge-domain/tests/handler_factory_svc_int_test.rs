#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — HandlerFactory trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::Handler;
use edge_domain::HandlerError;
use edge_domain::HandlerFactory;

struct Cfg {
    label: String,
}

struct LabelHandler {
    label: String,
}

impl HandlerFactory<Cfg> for LabelHandler {
    fn build(cfg: Cfg) -> Result<Self, HandlerError> {
        Ok(LabelHandler { label: cfg.label })
    }
}

#[async_trait]
impl Handler<(), String> for LabelHandler {
    fn id(&self) -> &str {
        &self.label
    }
    fn pattern(&self) -> &str {
        "*"
    }
    async fn execute(&self, _: ()) -> Result<String, HandlerError> {
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
    assert_eq!(h.execute(()).await.unwrap(), "echo");
}
