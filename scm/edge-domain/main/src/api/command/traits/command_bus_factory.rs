//! [`CommandBusFactory`] — constructor contract for command bus implementations.

use crate::api::command::types::direct_command_bus::DirectCommandBus;

/// Factory trait for the standard [`CommandBus`](crate::api::command::traits::command_bus::CommandBus) implementations.
pub trait CommandBusFactory {
    /// Construct the inline [`DirectCommandBus`] that dispatches commands without queuing.
    fn direct() -> DirectCommandBus {
        DirectCommandBus
    }
}
