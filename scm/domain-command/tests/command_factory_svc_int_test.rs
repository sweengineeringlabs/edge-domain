use edge_domain_command::{Command, CommandFactory, NoopCommand, StdCommandBusFactory};
use futures::executor::block_on;

/// @covers: CommandFactory::noop
#[test]
fn test_command_factory_trait_noop_returns_noop_command_happy() {
    let cmd: NoopCommand = <StdCommandBusFactory as CommandFactory>::noop();
    assert_eq!(cmd.name(), "command");
}

/// @covers: CommandFactory::noop
#[test]
fn test_command_factory_trait_noop_execute_always_succeeds_error() {
    let cmd = <StdCommandBusFactory as CommandFactory>::noop();
    assert!(block_on(cmd.execute()).is_ok());
}

/// @covers: CommandFactory::noop
#[test]
fn test_command_factory_trait_noop_result_is_copy_edge() {
    let a = <StdCommandBusFactory as CommandFactory>::noop();
    let b = a;
    let _ = (a, b);
}
