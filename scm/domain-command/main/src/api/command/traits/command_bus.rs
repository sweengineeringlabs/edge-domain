//! `CommandBus` trait — dispatches commands to their executors.

use futures::future::BoxFuture;

use crate::api::command::types::CommandDispatchRequest;
use crate::api::command::CommandError;

/// Dispatches [`Command`](super::command::Command) instances to their executors.
///
/// The bus decouples the caller from the command implementation.
pub trait CommandBus: Send + Sync {
    /// Dispatch a command. Returns `Err` if execution fails.
    fn dispatch(&self, req: CommandDispatchRequest) -> BoxFuture<'_, Result<(), CommandError>>;
}
