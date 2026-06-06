//! Command theme — port contracts.

#[allow(clippy::module_inception)]
pub mod command;
pub mod command_bus;

pub use command::Command;
pub use command_bus::CommandBus;
