//! `CommandBus` impl for the [`DirectCommandBus`] marker — inline dispatch, no queuing.

use futures::future::BoxFuture;

use crate::api::Command;
use crate::api::CommandBus;
use crate::api::CommandError;
use crate::api::DirectCommandBus;

impl CommandBus for DirectCommandBus {
    fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { cmd.execute().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::DirectCommandBus;

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
