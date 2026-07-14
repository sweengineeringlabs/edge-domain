#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Command trait is exported from the crate root.

use edge_application::Command;
use edge_application::CommandError;
use edge_application_command::{ExecutionRequest, NameRequest, NameResponse};

struct Succeed;
impl Command for Succeed {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "succeed".to_string(),
        })
    }
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Fail;
impl Command for Fail {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "fail".to_string(),
        })
    }
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::InvalidInput("bad".into())) })
    }
}

#[tokio::test]
async fn test_command_svc_facade_execute_returns_ok() {
    assert!(Succeed.execute(ExecutionRequest).await.is_ok());
}

#[tokio::test]
async fn test_command_svc_facade_execute_returns_err() {
    assert!(Fail.execute(ExecutionRequest).await.is_err());
}

#[test]
fn test_command_svc_facade_name_returns_identifier() {
    assert_eq!(Succeed.name(NameRequest).unwrap().name, "succeed");
}
