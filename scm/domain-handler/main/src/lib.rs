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
pub use saf::HandlerBootstrap;
pub use saf::HandlerContext;
pub use saf::HandlerError;
pub use saf::HandlerProvider;
pub use saf::HandlerRegistry;
pub use saf::InProcessHandlerRegistry;
pub use saf::NoopHandlerFactory;
pub use saf::HANDLER_BOOTSTRAP_SVC;
pub use saf::HANDLER_PROVIDER_SVC;
pub use saf::HANDLER_REGISTRY_SVC;
pub use saf::HANDLER_SVC;
