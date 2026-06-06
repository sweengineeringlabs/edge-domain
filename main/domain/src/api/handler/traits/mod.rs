//! Handler theme — port contracts.

#[allow(clippy::module_inception)]
pub mod handler;
pub mod handler_registry;

pub use handler::Handler;
pub use handler_registry::HandlerRegistry;
