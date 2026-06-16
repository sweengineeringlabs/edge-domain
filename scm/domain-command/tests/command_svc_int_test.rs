//! SAF facade tests — `Command` trait.

use edge_domain_command::{Command, CommandError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ping(String);
impl Command for Ping {
    fn name(&self) -> &str {
        &self.0
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Fails;
impl Command for Fails {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
    }
}

/// @covers: Command::name — configured name returned
#[test]
fn test_name_configured_value_returned_happy() {
    assert_eq!(Ping("create".into()).name(), "create");
}

/// @covers: Command::name — default impl returns "command"
#[test]
fn test_name_default_impl_returns_command_error() {
    assert_eq!(Fails.name(), "command");
}

/// @covers: Command::name — via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_returns_name_edge() {
    let c: &dyn Command = &Ping("x".into());
    assert_eq!(c.name(), "x");
}

/// @covers: Command::execute — success
#[test]
fn test_execute_ok_command_returns_ok_happy() {
    assert!(block_on(Ping("ok".into()).execute()).is_ok());
}

/// @covers: Command::execute — failure propagates
#[test]
fn test_execute_failing_command_returns_err_error() {
    assert!(block_on(Fails.execute()).is_err());
}

/// @covers: Command::execute — repeated execution is independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let c = Ping("x".into());
    assert!(block_on(c.execute()).is_ok());
    assert!(block_on(c.execute()).is_ok());
}
