//! `DirectCommandBus` — inline command dispatch with no queuing.

use futures::future::BoxFuture;

use crate::api::command::Command;
use crate::api::command::CommandBus;
use crate::api::command::CommandError;

/// Dispatches commands by calling `cmd.execute()` directly in the same task.
///
/// Suitable for synchronous in-process use cases. For distributed or
/// async-queue dispatch, replace with a bus implementation in the
/// infrastructure crate.
pub(crate) struct DirectCommandBus;

impl CommandBus for DirectCommandBus {
    fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { cmd.execute().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DirectCommandBusOk;
    impl Command for DirectCommandBusOk {
        fn name(&self) -> &str {
            "ok"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    struct DirectCommandBusErr;
    impl Command for DirectCommandBusErr {
        fn name(&self) -> &str {
            "err"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_command_returns_ok() {
        assert!(DirectCommandBus
            .dispatch(Box::new(DirectCommandBusOk))
            .await
            .is_ok());
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_err_command_returns_err() {
        assert!(DirectCommandBus
            .dispatch(Box::new(DirectCommandBusErr))
            .await
            .is_err());
    }
}
