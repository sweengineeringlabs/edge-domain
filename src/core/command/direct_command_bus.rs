//! `DirectCommandBus` — inline command dispatch with no queuing.

use async_trait::async_trait;

use crate::api::command::Command;
use crate::api::command_bus::CommandBus;
use crate::api::command_error::CommandError;

/// Dispatches commands by calling `cmd.execute()` directly in the same task.
///
/// Suitable for synchronous in-process use cases. For distributed or
/// async-queue dispatch, replace with a bus implementation in the
/// infrastructure crate.
pub(crate) struct DirectCommandBus;

#[async_trait]
impl CommandBus for DirectCommandBus {
    async fn dispatch(&self, cmd: Box<dyn Command>) -> Result<(), CommandError> {
        cmd.execute().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct OkCommand;
    #[async_trait]
    impl Command for OkCommand {
        fn name(&self) -> &str { "ok" }
        async fn execute(&self) -> Result<(), CommandError> { Ok(()) }
    }

    struct ErrCommand;
    #[async_trait]
    impl Command for ErrCommand {
        fn name(&self) -> &str { "err" }
        async fn execute(&self) -> Result<(), CommandError> {
            Err(CommandError::RuleViolation("blocked".into()))
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_command_returns_ok() {
        assert!(DirectCommandBus.dispatch(Box::new(OkCommand)).await.is_ok());
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_err_command_returns_err() {
        assert!(DirectCommandBus.dispatch(Box::new(ErrCommand)).await.is_err());
    }
}
