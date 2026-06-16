//! Integration coverage for `edge-domain-event` and `edge-domain-command` dependencies.
// @allow: no_mocks_in_integration

use edge_domain_command::{Command, CommandError};
use edge_domain_event::DomainEvent;
use futures::executor::block_on;
use futures::future::BoxFuture;

struct DepEvt;
impl DomainEvent for DepEvt {}

struct DepCmd;
impl Command for DepCmd {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[test]
fn test_domain_event_default_impls_are_callable_happy() {
    let e = DepEvt;
    assert_eq!(e.event_type(), "event");
    assert_eq!(e.aggregate_id(), "");
}

#[test]
fn test_command_execute_returns_ok_error() {
    let result = block_on(DepCmd.execute());
    assert!(result.is_ok());
}

#[test]
fn test_command_name_default_is_command_edge() {
    assert_eq!(DepCmd.name(), "command");
}
