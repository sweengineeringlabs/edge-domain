//! [`CommandBusFactory`] — constructor contract for command bus implementations.

use crate::api::command::types::{DirectCommandBus, NoopCommand, StdCommandBusFactory};

/// Factory trait for the standard `CommandBus` implementations.
pub trait CommandBusFactory {
    /// Construct the inline `DirectCommandBus` that dispatches commands without queuing.
    fn direct() -> DirectCommandBus {
        DirectCommandBus
    }

    /// Return a no-op command instance for structural compliance.
    fn noop_command() -> NoopCommand {
        NoopCommand
    }

    /// Return the standard command-bus-factory instance.
    fn std_factory() -> StdCommandBusFactory {
        StdCommandBusFactory
    }
}
