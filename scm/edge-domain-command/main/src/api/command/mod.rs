//! `Command` theme — CQRS write-side contracts.

pub mod direct_command_bus;
pub mod errors;
pub mod traits;
pub mod types;

pub use direct_command_bus::DirectCommandBus;
pub use errors::CommandError;
pub use traits::{Command, CommandBus, CommandBusFactory};
pub use types::{NoopCommand, StdCommandBusFactory};
