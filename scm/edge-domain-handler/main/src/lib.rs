//! # edge-domain-handler
//!
//! The `Handler` port contract — request/response execution units with registry and context.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::EchoHandler;
pub use saf::Handler;
pub use saf::HandlerError;
pub use saf::HandlerFactory;
pub use saf::HandlerProvider;
pub use saf::HandlerRegistry;
pub use saf::InProcessHandlerRegistry;
