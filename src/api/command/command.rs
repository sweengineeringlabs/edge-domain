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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_is_object_safe() {
        fn _assert(_: &dyn Command) {}
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
    async fn test_execute_returns_ok() {
        assert!(NoopCommand.execute().await.is_ok());
    }
}
