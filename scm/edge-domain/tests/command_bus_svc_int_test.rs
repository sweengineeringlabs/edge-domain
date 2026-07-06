#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — CommandBus is exported from the crate root.

use edge_domain::Command;
use edge_domain::CommandBus;
use edge_domain::CommandError;
use edge_domain::Domain;
use edge_domain_command::{CommandDispatchRequest, ExecutionRequest, NameRequest, NameResponse};

struct Noop;
impl Command for Noop {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "noop".to_string(),
        })
    }
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test]
async fn test_command_bus_svc_facade_dispatch_ok_command() {
    let bus = Domain::direct_command_bus();
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(Noop)
        })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_command_bus_svc_facade_dispatch_failing_command() {
    struct Bad;
    impl Command for Bad {
        fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
            Ok(NameResponse {
                name: "bad".to_string(),
            })
        }
        fn execute(
            &self,
            _req: ExecutionRequest,
        ) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Err(CommandError::InvalidInput("rejected".into())) })
        }
    }
    let bus = Domain::direct_command_bus();
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(Bad)
        })
        .await
        .is_err());
}
