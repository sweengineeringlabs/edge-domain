//! [`CommandBusBootstrap`] — constructor contract for command bus implementations.

use std::sync::Arc;

use crate::api::command::traits::CommandBus;
use crate::api::command::types::{
    DirectCommandBus, LoggingCommandBus, NoopCommandBus, StdCommandBusFactory,
};

/// Bootstrap trait for the standard `CommandBus` implementations.
pub trait CommandBusBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "command_bus"
    }

    /// Construct the inline `DirectCommandBus` that dispatches commands without queuing.
    fn direct() -> DirectCommandBus where Self: Sized {
        DirectCommandBus
    }

    /// Return the standard command-bus-factory instance.
    fn std_factory() -> StdCommandBusFactory where Self: Sized {
        StdCommandBusFactory
    }

    /// Construct a `NoopCommandBus` that silently discards every command.
    fn noop_bus() -> NoopCommandBus where Self: Sized {
        NoopCommandBus
    }

    /// Wrap `inner` with a `LoggingCommandBus` that records dispatch outcomes via `tracing`.
    fn logging(inner: Arc<dyn CommandBus>) -> LoggingCommandBus where Self: Sized {
        LoggingCommandBus { inner }
    }
}
