//! `CommandBus` trait — dispatches commands to their executors.

use async_trait::async_trait;

use crate::api::command::Command;
use crate::api::command_error::CommandError;

/// Dispatches [`Command`] instances to their executors.
///
/// The bus decouples the caller from the command implementation.
/// A simple in-process bus calls `cmd.execute()` directly; a distributed
/// bus may serialize and route to a remote worker.
///
/// ```rust,ignore
/// #[async_trait]
/// impl CommandBus for DirectCommandBus {
///     async fn dispatch(&self, cmd: Box<dyn Command>) -> Result<(), CommandError> {
///         cmd.execute().await
///     }
/// }
/// ```
#[async_trait]
pub trait CommandBus: Send + Sync {
    /// Dispatch a command. Returns `Err` if execution fails.
    async fn dispatch(&self, cmd: Box<dyn Command>) -> Result<(), CommandError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_bus_is_object_safe() {
        fn _assert(_: &dyn CommandBus) {}
    }
}
