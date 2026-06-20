use edge_domain_command::{Command, CommandBootstrap, NoopCommand, StdCommandBusFactory};
use futures::executor::block_on;

/// @covers: CommandBootstrap::noop
#[test]
fn test_command_factory_trait_noop_returns_noop_command_happy() {
    let cmd: NoopCommand = <StdCommandBusFactory as CommandBootstrap>::noop();
    assert_eq!(cmd.name(), "command");
}

/// @covers: CommandBootstrap::noop
#[test]
fn test_command_factory_trait_noop_execute_always_succeeds_error() {
    let cmd = <StdCommandBusFactory as CommandBootstrap>::noop();
    assert!(block_on(cmd.execute()).is_ok());
}

/// @covers: CommandBootstrap::noop
#[test]
fn test_command_factory_trait_noop_result_is_copy_edge() {
    let a = <StdCommandBusFactory as CommandBootstrap>::noop();
    let b = a;
    let _ = (a, b);
}
