#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Command trait is exported from the crate root.

use edge_domain::Command;
use edge_domain::CommandError;

struct Succeed;
impl Command for Succeed {
    fn name(&self) -> &str {
        "succeed"
    }
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Fail;
impl Command for Fail {
    fn name(&self) -> &str {
        "fail"
    }
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::InvalidInput("bad".into())) })
    }
}

#[tokio::test]
async fn test_command_svc_facade_execute_returns_ok() {
    assert!(Succeed.execute().await.is_ok());
}

#[tokio::test]
async fn test_command_svc_facade_execute_returns_err() {
    assert!(Fail.execute().await.is_err());
}

#[test]
fn test_command_svc_facade_name_returns_identifier() {
    assert_eq!(Succeed.name(), "succeed");
}
