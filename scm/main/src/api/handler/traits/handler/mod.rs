//! Handler theme — port contracts owned by the domain.

#[allow(clippy::module_inception)]
pub mod handler;
pub mod handler_factory;
pub mod handler_registry;

pub use handler::Handler;
pub use handler_factory::HandlerFactory;
pub use handler_registry::HandlerRegistry;
