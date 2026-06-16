//! [`CommandBusFactory`] — constructor contract for command bus implementations.

use std::sync::Arc;

use crate::api::command::traits::CommandBus;
use crate::api::command::types::{
    DirectCommandBus, LoggingCommandBus, NoopCommandBus, StdCommandBusFactory,
};

/// Factory trait for the standard `CommandBus` implementations.
pub trait CommandBusFactory {
    /// Construct the inline `DirectCommandBus` that dispatches commands without queuing.
    fn direct() -> DirectCommandBus {
        DirectCommandBus
    }

    /// Return the standard command-bus-factory instance.
    fn std_factory() -> StdCommandBusFactory {
        StdCommandBusFactory
    }

    /// Construct a `NoopCommandBus` that silently discards every command.
    fn noop_bus() -> NoopCommandBus {
        NoopCommandBus
    }

    /// Wrap `inner` with a `LoggingCommandBus` that records dispatch outcomes via `tracing`.
    fn logging(inner: Arc<dyn CommandBus>) -> LoggingCommandBus {
        LoggingCommandBus { inner }
    }
}
