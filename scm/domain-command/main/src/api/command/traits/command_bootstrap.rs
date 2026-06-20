//! `CommandBootstrap` — constructor contract for [`Command`] implementations.

use crate::api::command::types::NoopCommand;

/// Bootstrap trait for standard [`Command`](super::command::Command) implementations.
///
/// Separates command construction from bus construction — `CommandBootstrap`
/// produces [`Command`] values; [`CommandBusBootstrap`](super::command_bus_bootstrap::CommandBusBootstrap)
/// produces [`CommandBus`](super::command_bus::CommandBus) implementations.
pub trait CommandBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "command"
    }

    /// Construct a [`NoopCommand`] — a structural placeholder that always succeeds.
    fn noop() -> NoopCommand where Self: Sized {
        NoopCommand
    }
}
