//! Integration tests for [`NoopSagaCommand`].
// @allow: no_mocks_in_integration

use edge_domain_saga::{Command, NoopSagaCommand};
use futures::executor::block_on;

/// @covers: execute
#[test]
fn test_execute_noop_saga_command_returns_ok_happy() {
    let cmd = NoopSagaCommand;
    let result = block_on(cmd.execute());
    assert!(result.is_ok());
}

/// @covers: name
#[test]
fn test_name_noop_saga_command_returns_default_error() {
    // Verifies the Command default impl is inherited, not accidentally overridden
    let cmd = NoopSagaCommand;
    assert_eq!(cmd.name(), "command");
}

/// @covers: execute
#[test]
fn test_execute_noop_saga_command_can_be_called_multiple_times_edge() {
    let cmd = NoopSagaCommand;
    let r1 = block_on(cmd.execute());
    let r2 = block_on(cmd.execute());
    assert!(r1.is_ok());
    assert!(r2.is_ok());
}
