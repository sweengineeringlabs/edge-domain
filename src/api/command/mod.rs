//! `Command` module — write operations that mutate domain state.

#[allow(clippy::module_inception)]
pub mod command;
pub mod command_bus;
pub mod command_error;
pub mod direct_command_bus;

pub use command::Command;
pub use command_bus::CommandBus;
pub use command_error::CommandError;
