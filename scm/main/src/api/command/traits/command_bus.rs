//! `CommandBus` trait — dispatches commands to their executors.

use futures::future::BoxFuture;

use super::command::Command;
use crate::api::command::CommandError;

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
