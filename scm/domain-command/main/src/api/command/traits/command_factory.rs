//! `CommandFactory` — constructor contract for [`Command`] implementations.

use crate::api::command::types::NoopCommand;

/// Factory trait for standard [`Command`](super::command::Command) implementations.
///
/// Separates command construction from bus construction — `CommandFactory`
/// produces [`Command`] values; [`CommandBusFactory`](super::command_bus_factory::CommandBusFactory)
/// produces [`CommandBus`](super::command_bus::CommandBus) implementations.
pub trait CommandFactory {
    /// Construct a [`NoopCommand`] — a structural placeholder that always succeeds.
    fn noop() -> NoopCommand {
        NoopCommand
    }
}
