//! [`CommandBusFactory`] — constructor contract for command bus implementations.

use crate::api::command::types::{DirectCommandBus, StdCommandBusFactory};

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
}
