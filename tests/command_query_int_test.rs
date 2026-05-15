//! Integration tests for `Command`, `Query`, `CommandBus`, and `QueryBus`.

use std::sync::Arc;
use async_trait::async_trait;
use edge_domain::{Command, CommandBus, CommandError, Query, QueryBus};

// ── Command fixtures ─────────────────────────────────────────────────────────

struct NoopCommand;

#[async_trait]
impl Command for NoopCommand {
    fn name(&self) -> &str { "noop" }
    async fn execute(&self) -> Result<(), CommandError> { Ok(()) }
}

struct FailingCommand;

#[async_trait]
impl Command for FailingCommand {
    fn name(&self) -> &str { "failing" }
    async fn execute(&self) -> Result<(), CommandError> {
        Err(CommandError::RuleViolation("blocked".into()))
    }
}

// ── Query fixtures ───────────────────────────────────────────────────────────

struct EchoQuery { value: String }

#[async_trait]
impl Query<String> for EchoQuery {
    fn name(&self) -> &str { "echo" }
    async fn execute(&self) -> Result<String, CommandError> { Ok(self.value.clone()) }
}

// ── CommandBus fixture ───────────────────────────────────────────────────────

struct DirectCommandBus;

#[async_trait]
impl CommandBus for DirectCommandBus {
    async fn dispatch(&self, cmd: Box<dyn Command>) -> Result<(), CommandError> {
        cmd.execute().await
    }
}

// ── QueryBus fixture ─────────────────────────────────────────────────────────

struct DirectQueryBus;

#[async_trait]
impl QueryBus<String> for DirectQueryBus {
    async fn dispatch(&self, query: Box<dyn Query<String>>) -> Result<String, CommandError> {
        query.execute().await
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

/// @covers: Command::execute
#[tokio::test]
async fn test_command_trait_execute_returns_ok_on_success() {
    assert!(NoopCommand.execute().await.is_ok());
}

/// @covers: Command::execute
#[tokio::test]
async fn test_command_trait_execute_returns_err_on_failure() {
    assert!(FailingCommand.execute().await.is_err());
}

/// @covers: Command::name
#[test]
fn test_command_trait_name_returns_stable_identifier() {
    assert_eq!(NoopCommand.name(), "noop");
}

/// @covers: Query::execute
#[tokio::test]
async fn test_query_trait_execute_returns_result_without_mutation() {
    let q = EchoQuery { value: "pong".into() };
    assert_eq!(q.execute().await.unwrap(), "pong");
}

/// @covers: CommandBus::dispatch
#[tokio::test]
async fn test_command_bus_trait_dispatch_delegates_to_command_execute() {
    let bus: Arc<dyn CommandBus> = Arc::new(DirectCommandBus);
    assert!(bus.dispatch(Box::new(NoopCommand)).await.is_ok());
}

/// @covers: CommandBus::dispatch
#[tokio::test]
async fn test_command_bus_trait_dispatch_propagates_command_error() {
    let bus: Arc<dyn CommandBus> = Arc::new(DirectCommandBus);
    assert!(bus.dispatch(Box::new(FailingCommand)).await.is_err());
}

/// @covers: QueryBus::dispatch
#[tokio::test]
async fn test_query_bus_trait_dispatch_returns_query_result() {
    let bus: Arc<dyn QueryBus<String>> = Arc::new(DirectQueryBus);
    let result = bus.dispatch(Box::new(EchoQuery { value: "hello".into() })).await.unwrap();
    assert_eq!(result, "hello");
}
