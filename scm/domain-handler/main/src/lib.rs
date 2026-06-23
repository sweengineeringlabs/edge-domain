//! # edge-domain-handler
//!
//! The `Handler` port contract — request/response execution units with registry and context.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

// Trait contracts through SAF
pub use saf::Handler;
pub use saf::HandlerBootstrap;
pub use saf::HandlerProvider;
pub use saf::HandlerRegistry;
pub use saf::HANDLER_BOOTSTRAP_SVC;
pub use saf::HANDLER_PROVIDER_SVC;
pub use saf::HANDLER_REGISTRY_SVC;
pub use saf::HANDLER_SVC;

// Concrete types from api (needed for trait method signatures)
pub use api::EchoHandler;
pub use api::HandlerContext;
pub use api::HandlerError;
pub use api::InProcessHandlerRegistry;
pub use api::NoopHandlerFactory;
