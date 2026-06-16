//! `CommandBus` impl for the [`DirectCommandBus`] marker — inline dispatch, no queuing.

use futures::future::BoxFuture;

use crate::api::command::types::DirectCommandBus;
use crate::api::command::Command;
use crate::api::command::CommandBus;
use crate::api::command::CommandError;

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

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_command_returns_ok() {
        assert!(DirectCommandBus
            .dispatch(Box::new(DirectCommandBusOk))
            .await
            .is_ok());
    }
}
