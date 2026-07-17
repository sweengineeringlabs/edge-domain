//! Integration tests — `LocalCommandAsForeign` bridge, exercised indirectly through the
//! `CommandBus::dispatch` blanket impl (the only public entry point that constructs it).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    Command, CommandBus, CommandDispatchRequest, CommandExecutionRequest, HandlerError,
};
use futures::executor::block_on;

struct OkCmd;
impl Command for OkCmd {
    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>>
    {
        Box::pin(async { Ok(()) })
    }
}

struct FailingCmd;
impl Command for FailingCmd {
    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>>
    {
        Box::pin(async { Err(HandlerError::ExecutionFailed("fail".into())) })
    }
}

/// @covers: LocalCommandAsForeign — wraps a successful local command for the real bus
#[test]
fn test_local_command_as_foreign_wraps_ok_command_happy() {
    let bus = DirectCommandBus;
    let result = block_on(CommandBus::dispatch(
        &bus,
        CommandDispatchRequest {
            command: Box::new(OkCmd),
        },
    ));
    assert_eq!(result, Ok(()));
}

/// @covers: LocalCommandAsForeign — propagates a failing local command's error through the real bus
#[test]
fn test_local_command_as_foreign_wraps_failing_command_error() {
    let bus = DirectCommandBus;
    let result = block_on(CommandBus::dispatch(
        &bus,
        CommandDispatchRequest {
            command: Box::new(FailingCmd),
        },
    ));
    assert!(result.is_err());
}

/// @covers: LocalCommandAsForeign — the default name is preserved through the bridge
#[test]
fn test_local_command_as_foreign_default_name_preserved_edge() {
    assert_eq!(
        OkCmd
            .name(edge_application_handler::CommandNameRequest)
            .unwrap()
            .name,
        "command"
    );
}
