//! `Command` trait — a write operation that mutates domain state.

pub mod command_bus;
pub mod command_error;

pub use command_bus::CommandBus;
pub use command_error::CommandError;

use async_trait::async_trait;

/// A named write operation that mutates domain state and returns no value.
///
/// Commands are the write side of the CQRS split.  They express intent
/// ("create order", "cancel subscription") and return only success or failure.
///
/// ```rust,ignore
/// struct CreateOrder { customer_id: String, items: Vec<Item> }
///
/// #[async_trait]
/// impl Command for CreateOrder {
///     fn name(&self) -> &str { "create-order" }
///     async fn execute(&self) -> Result<(), CommandError> {
///         // mutate state
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Command: Send + Sync {
    /// Stable name identifying this command type.
    fn name(&self) -> &str;

    /// Execute the command, mutating domain state.
    async fn execute(&self) -> Result<(), CommandError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_is_object_safe() {
        fn _assert(_: &dyn Command) {}
    }
}
