#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Handler trait is exported from the crate root.

use async_trait::async_trait;
use edge_domain::Handler;
use edge_domain::HandlerError;

struct Doubler;

#[async_trait]
impl Handler<i32, i32> for Doubler {
    fn id(&self) -> &str {
        "doubler"
    }
    fn pattern(&self) -> &str {
        "*"
    }
    async fn execute(&self, req: i32) -> Result<i32, HandlerError> {
        Ok(req * 2)
    }
}

#[tokio::test]
async fn test_handler_svc_facade_execute_doubles_input() {
    assert_eq!(Doubler.execute(21).await.unwrap(), 42);
}

#[tokio::test]
async fn test_handler_svc_facade_health_check_defaults_true() {
    assert!(Doubler.health_check().await);
}
