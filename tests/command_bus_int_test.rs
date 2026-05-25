//! Tests for CommandBus trait
use edge_domain::{Command, CommandBus, CommandError};
use futures::future::BoxFuture;
use std::sync::Arc;

struct OkCommand;
impl Command for OkCommand {
    fn name(&self) -> &str {
        "ok"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test]
async fn test_command_bus_dispatch() {
    let bus = edge_domain::direct_command_bus();
    let result = bus.dispatch(Box::new(OkCommand)).await;
    assert!(result.is_ok());
}
