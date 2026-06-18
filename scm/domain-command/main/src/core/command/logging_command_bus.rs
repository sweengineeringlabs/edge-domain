//! `CommandBus` impl for `LoggingCommandBus` — delegates to inner bus, logs via `tracing`.

use futures::future::BoxFuture;

use crate::api::Command;
use crate::api::CommandBus;
use crate::api::CommandError;
use crate::api::LoggingCommandBus;

impl CommandBus for LoggingCommandBus {
    fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
        let name = cmd.name().to_owned();
        let inner = self.inner.clone();
        Box::pin(async move {
            let result = inner.dispatch(cmd).await;
            match &result {
                Ok(()) => tracing::debug!(command = %name, "command dispatched ok"),
                Err(e) => tracing::debug!(command = %name, error = %e, "command dispatch error"),
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;

    use super::*;
    use crate::api::DirectCommandBus;
    use crate::api::NoopCommandBus;

    struct LoggingCommandBusOk;
    impl Command for LoggingCommandBusOk {
        fn name(&self) -> &str {
            "ok-cmd"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    struct LoggingCommandBusErr;
    impl Command for LoggingCommandBusErr {
        fn name(&self) -> &str {
            "err-cmd"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_command_returns_ok() {
        let bus = LoggingCommandBus { inner: Arc::new(NoopCommandBus) };
        assert!(bus.dispatch(Box::new(LoggingCommandBusOk)).await.is_ok());
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_error_command_propagates_err() {
        let bus = LoggingCommandBus { inner: Arc::new(DirectCommandBus) };
        assert!(bus.dispatch(Box::new(LoggingCommandBusErr)).await.is_err());
    }
}
