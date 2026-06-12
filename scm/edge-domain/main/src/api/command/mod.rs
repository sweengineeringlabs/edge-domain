//! `Command` theme — write operations that mutate domain state.

pub mod direct_command_bus;
pub mod errors;
pub mod traits;
pub mod types;

pub use errors::CommandError;
pub use traits::{Command, CommandBus, CommandBusFactory, DirectCommandBus};
