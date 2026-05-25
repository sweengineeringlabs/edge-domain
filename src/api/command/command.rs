//! `Command` trait — a write operation that mutates domain state.

use futures::future::BoxFuture;

use super::command_error::CommandError;

/// A named write operation that mutates domain state and returns no value.
///
/// Commands are the write side of the CQRS split.  They express intent
/// ("create order", "cancel subscription") and return only success or failure.
///
/// ```rust,ignore
/// struct CreateOrder { customer_id: String, items: Vec<Item> }
///
/// impl Command for CreateOrder {
///     fn name(&self) -> &str { "create-order" }
///     fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
///         Box::pin(async move {
///             // mutate state
///             Ok(())
///         })
///     }
/// }
/// ```
pub trait Command: Send + Sync {
    /// Stable name identifying this command type.
    fn name(&self) -> &str;

    /// Execute the command, mutating domain state.
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>>;
}


