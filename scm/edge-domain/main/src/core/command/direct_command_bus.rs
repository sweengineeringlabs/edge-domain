//! `DirectCommandBus` — inline command dispatch with no queuing.

use futures::future::BoxFuture;

use crate::api::Command;
use crate::api::CommandBus;
use crate::api::CommandError;
use crate::api::CommandBusFactory;

/// Dispatches commands by calling `cmd.execute()` directly in the same task.
///
/// Suitable for synchronous in-process use cases. For distributed or
/// async-queue dispatch, replace with a bus implementation in the
/// infrastructure crate.
pub(crate) struct DirectCommandBus;

impl CommandBusFactory for DirectCommandBus {}

// impl DomainFactory for DefaultDomainFactory (edge-domain-domain sub-crate)
// impl EventFactory for DefaultEventFactory
// impl Handler for DefaultHandler
// impl HandlerFactory for DefaultHandler
// impl HandlerProvider for DefaultHandlerProvider
// impl RepositoryFactory for DefaultRepositoryFactory
// impl Saga for NoopSaga
// impl Service for NoopService
// impl ServiceRegistry for InProcessServiceRegistry
// impl Snapshot for NoopSnapshot
// impl Validator for AlwaysValid (edge-domain-validator sub-crate)
// impl ValueObject for NoopValueObject (edge-domain-valueobject sub-crate)
// impl ValueObjectFactory for NoopValueObjectFactory (edge-domain-valueobject sub-crate)

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

    struct DirectCommandBusErr;
    impl Command for DirectCommandBusErr {
        fn name(&self) -> &str {
            "err"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
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

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_err_command_returns_err() {
        assert!(DirectCommandBus
            .dispatch(Box::new(DirectCommandBusErr))
            .await
            .is_err());
    }
}
