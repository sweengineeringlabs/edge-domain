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
pub use saf::DirectCommandBus;
