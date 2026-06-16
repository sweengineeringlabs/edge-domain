//! # edge-domain-command
//!
//! The `Command` port contract — CQRS write-side with CommandBus and DirectCommandBus.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Command;
pub use saf::CommandBus;
pub use saf::CommandBusFactory;
pub use saf::CommandError;
pub use saf::CommandFactory;
pub use saf::DirectCommandBus;
pub use saf::LoggingCommandBus;
pub use saf::NoopCommand;
pub use saf::NoopCommandBus;
pub use saf::StdCommandBusFactory;
pub use saf::COMMAND_BUS_FACTORY_SVC;
pub use saf::COMMAND_BUS_SVC;
pub use saf::COMMAND_FACTORY_SVC;
pub use saf::COMMAND_SVC;
