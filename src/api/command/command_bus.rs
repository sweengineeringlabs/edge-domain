//! `CommandBus` trait — dispatches commands to their executors.

use futures::future::BoxFuture;

use super::command::Command;
use super::command_error::CommandError;

/// Dispatches [`Command`] instances to their executors.
///
/// The bus decouples the caller from the command implementation.
/// A simple in-process bus calls `cmd.execute()` directly; a distributed
/// bus may serialize and route to a remote worker.
///
/// ```rust,ignore
/// impl CommandBus for DirectCommandBus {
///     fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
///         Box::pin(async move { cmd.execute().await })
///     }
/// }
/// ```
pub trait CommandBus: Send + Sync {
    /// Dispatch a command. Returns `Err` if execution fails.
    fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_bus_is_object_safe() {
        fn _assert(_: &dyn CommandBus) {}
    }

    struct NoopCommandBus;
    impl CommandBus for NoopCommandBus {
        fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async move { cmd.execute().await })
        }
    }

    struct NoopCommand;
    impl Command for NoopCommand {
        fn name(&self) -> &str {
            "noop"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_dispatch_delegates_to_command() {
        assert!(NoopCommandBus
            .dispatch(Box::new(NoopCommand))
            .await
            .is_ok());
    }
}
