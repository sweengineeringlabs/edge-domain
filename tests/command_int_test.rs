//! Tests for Command trait
use edge_domain::Command;
use futures::future::BoxFuture;

struct TestCommand;
impl Command for TestCommand {
    fn name(&self) -> &str {
        "test"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[test]
fn test_command_name() {
    let cmd = TestCommand;
    assert_eq!(cmd.name(), "test");
}
