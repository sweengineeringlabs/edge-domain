//! Integration tests — blanket bridge from `edge_application_command`'s `Command`/`CommandBus`
//! traits to the local `domain-handler` decoupling boundaries.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    Command, CommandBus, CommandBusAdapter, CommandDispatchRequest, CommandExecutionRequest,
    CommandNameRequest,
};
use futures::executor::block_on;

struct OkCmd;
impl edge_application_command::Command for OkCmd {
    fn execute(
        &self,
        _req: edge_application_command::ExecutionRequest,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<(), edge_application_command::CommandError>>
                + Send
                + '_,
        >,
    > {
        Box::pin(async { Ok(()) })
    }
}

/// @covers: Command::name — blanket bridge uses the foreign default name
#[test]
fn test_command_bridge_name_default_returns_command_happy() {
    assert_eq!(
        Command::name(&OkCmd, CommandNameRequest).unwrap().name,
        "command"
    );
}

/// @covers: Command::execute — blanket bridge propagates a successful foreign execution
#[test]
fn test_command_bridge_execute_ok_returns_ok_error() {
    assert_eq!(
        block_on(Command::execute(&OkCmd, CommandExecutionRequest)),
        Ok(())
    );
}

/// @covers: CommandBus::dispatch — blanket bridge dispatches through a real foreign bus
#[test]
fn test_command_bus_bridge_dispatches_via_real_bus_edge() {
    let bus = DirectCommandBus;
    let result = block_on(CommandBus::dispatch(
        &bus,
        CommandDispatchRequest {
            command: Box::new(OkCmd),
        },
    ));
    assert_eq!(result, Ok(()));
}

/// @covers: CommandBus::dispatch — CommandBusAdapter bridges an already-erased foreign bus
#[test]
fn test_command_bus_adapter_bridges_erased_reference_edge() {
    let bus = DirectCommandBus;
    let erased: &dyn edge_application_command::CommandBus = &bus;
    let adapter = CommandBusAdapter(erased);
    let result = block_on(CommandBus::dispatch(
        &adapter,
        CommandDispatchRequest {
            command: Box::new(OkCmd),
        },
    ));
    assert_eq!(result, Ok(()));
}
