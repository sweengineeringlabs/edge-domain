//! `Command` module — write operations that mutate domain state.

#[allow(clippy::module_inception)]
pub mod command;
pub mod command_bus;
pub mod direct_command_bus;

pub use crate::api::error::CommandError;
pub use command::Command;
pub use command_bus::CommandBus;
