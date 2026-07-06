#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{
    Command, CommandBootstrap, ExecutionRequest, NameRequest, NoopCommand, StdCommandBusFactory,
};
use futures::executor::block_on;

/// @covers: CommandBootstrap::noop
#[test]
fn test_command_factory_trait_noop_returns_noop_command_happy() {
    let cmd: NoopCommand = <StdCommandBusFactory as CommandBootstrap>::noop();
    let response = cmd.name(NameRequest).expect("name should succeed");
    assert_eq!(response.name, "command");
}

/// @covers: CommandBootstrap::noop
#[test]
fn test_command_factory_trait_noop_execute_always_succeeds_error() {
    let cmd = <StdCommandBusFactory as CommandBootstrap>::noop();
    assert_eq!(block_on(cmd.execute(ExecutionRequest)), Ok(()));
}

/// @covers: CommandBootstrap::noop
#[test]
fn test_command_factory_trait_noop_result_is_copy_edge() {
    let a = <StdCommandBusFactory as CommandBootstrap>::noop();
    let b = a;
    // Both a and b should still be valid, proving Copy semantics
    assert_eq!(
        a.name(NameRequest).expect("name should succeed"),
        b.name(NameRequest).expect("name should succeed")
    );
}
