//! `Command` trait — a write operation that mutates domain state.

use futures::future::BoxFuture;

use crate::api::command::CommandError;

/// A named write operation that mutates domain state and returns no value.
///
/// Commands are the write side of the CQRS split. They express intent
/// ("create order", "cancel subscription") and return only success or failure.
pub trait Command: Send + Sync {
    /// Stable name identifying this command type.
    fn name(&self) -> &str {
        "command"
    }

    /// Execute the command, mutating domain state.
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>>;
}
